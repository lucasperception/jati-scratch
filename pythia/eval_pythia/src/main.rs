use std::{
    collections::{BTreeSet, VecDeque},
    ops::RangeInclusive,
    path::{Path, PathBuf},
    process::{exit},
};

mod phyml_runner;
mod pythia_runner;
mod raxml_runner;

#[derive(Clone, Debug)]
struct DataSet {
    path: PathBuf,
    n_seqs: usize,
    seq_len: usize,
    difficulty: f64,
    raxml_runtime: f64,
    phyml_runtime: f64,
}

    dimensions: DataSetDimensions,
    difficulty: Option<f64>,
}

impl DataSet {
    fn into_flat(self) -> FlatDataset {
        FlatDataset {
            path: self.path.to_string_lossy().to_string(),
            difficulty: self.difficulty.unwrap_or(f64::NAN),
            n_seqs: self.dimensions.n_seqs,
            seq_len_min: *self.dimensions.seq_len.start(),
            seq_len_max: *self.dimensions.seq_len.end(),
            alphabet: self.dimensions.alphabet,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct FlatDataset {
    path: String,
    difficulty: f64,
    n_seqs: usize,
    seq_len_min: usize,
    seq_len_max: usize,
    alphabet: Alphabet,
}

const FILE_EXTENSIONS: &[&str] = &[".fas", ".fna", ".fasta", ".aln"];

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let [_exe, dir_arg] = &args[..] else {
        eprintln!("usage: <exe> path-to-dir-with-fasta-files");
        exit(1);
    };

    let fasta_file_paths = std::fs::read_dir(dir_arg)
        .expect("path should exist")
        .filter_map(|entry| {
            let entry = entry.expect("should be able to access files in dir");
            let file_name = entry
                .file_name()
                .into_string()
                .expect("filenames should be utf-8 encodable");
            if entry.file_type().is_ok_and(|file_type| file_type.is_file())
                && FILE_EXTENSIONS.iter().any(|ext| file_name.ends_with(ext))
            {
                Some(entry.path())
            } else {
                None
            }
        });
    let mut evaluated_datasets = Vec::new();
    let mut failed_datasets = Vec::new();

    for fasta_file_path in fasta_file_paths {
        let (n_seqs, seq_len) = read_n_seqs_and_seq_len(&fasta_file_path);
        println!("Evaluating {fasta_file_path:?} with dimensions ({n_seqs},{seq_len})");
        match pythia_runner::run(&fasta_file_path) {
            Some(difficulty) => {
                let raxml_runtime = raxml_runner::run(&fasta_file_path).unwrap_or_else(|| {-1.0});
                let phyml_runtime = phyml_runner::run(&fasta_file_path).unwrap_or_else(|| {-1.0});
                evaluated_datasets.push(DataSet {
                    path: fasta_file_path,
                    n_seqs,
                    seq_len,
                    difficulty,
                    raxml_runtime,
                    phyml_runtime,
                });
            }
            None => {
                eprintln!("Failed to estimate difficulty for the dataset {fasta_file_path:?}");
                failed_datasets.push(fasta_file_path);
            }
        };
    let do_measure_difficulty = false;

    let mut dir_queue = VecDeque::from([PathBuf::from(dir_arg)]);
    let mut discovered_filepaths = vec![];
    while let Some(dir) = dir_queue.pop_front() {
        let fasta_file_paths = std::fs::read_dir(dir)
            .expect("path should exist")
            .filter_map(|entry| {
                let entry = entry.expect("should be able to access files in dir");
                let file_name = entry
                    .file_name()
                    .into_string()
                    .expect("filenames should be utf-8 encodable");
                match entry
                    .file_type()
                    .expect("failed to determine file type walking directory tree")
                {
                    ft if ft.is_file()
                        && FILE_EXTENSIONS.iter().any(|ext| file_name.ends_with(ext)) =>
                    {
                        Some(entry.path())
                    }
                    ft if ft.is_dir() => {
                        dir_queue.push_front(entry.path());
                        None
                    }
                    _ => None,
                }
            });
        discovered_filepaths.extend(fasta_file_paths);
    }
    let mut evaluated_datasets: Vec<_> = discovered_filepaths
        .into_par_iter()
        .map(|fasta_file_path| {
            let dimensions = read_n_seqs_and_seq_len(&fasta_file_path);
            let difficulty = do_measure_difficulty.then(|| {
                difficulty_from_pythia(&fasta_file_path).unwrap_or_else(|| {
                    eprintln!("{fasta_file_path:?} failed to predict difficulty");
                    -1.
                })
            });
            let captured = DataSet {
                path: fasta_file_path,
                dimensions,
                difficulty,
            };
            println!("evaluated dataset: {captured:?}");
            captured
        })
        .collect();

    }
    println!(
        "successfully evaluated a total of {} datasets",
        evaluated_datasets.len()
    );
    println!("most difficult datasets sorted in order");
    evaluated_datasets.sort_unstable_by(|left, right| {
        right
            .difficulty
            .partial_cmp(&left.difficulty)
            .expect("difficulty should not have NaN's")
    });
    println!("{:?}", &evaluated_datasets);
    let mut csv_writer =
        csv::Writer::from_path("eval.csv").expect("failed to open csv output file");
    evaluated_datasets.into_iter().for_each(|dataset| {
        csv_writer
            .serialize(dataset.into_flat())
            .expect("failed to write dataset to csv")
    });
    csv_writer.flush().expect("failed to flush output csv");
}

fn difficulty_from_pythia(path: &Path) -> Option<f64> {
    let pythia_output = Command::new("python")
        .arg("predictor.py")
        .arg(path)
        .output()
        .expect("python script should be runnable");
    if pythia_output.status.success() {
        let stdout =
            String::from_utf8(pythia_output.stdout).expect("stdout of predictor is not valid utf8");
        let stdout = stdout.trim_end();
        let difficulty: f64 = match stdout.parse() {
            Ok(v) => v,
            Err(error) => panic!("failed to parse stdout '{stdout}' as a float: {error:?}"),
        };
        Some(difficulty)
    } else {
        eprintln!(
            "pythia failed: {}",
            String::from_utf8_lossy(&pythia_output.stderr)
        );
        None
    }
}

fn read_n_seqs_and_seq_len(path: &Path) -> DataSetDimensions {
    let mut reader = seq_io::fasta::Reader::from_path(path).expect("path should point to a file");

    let mut n_seqs = 0;
    let mut seq_lens = BTreeSet::new();
    let mut alphabet = Alphabet::Nucleotide;
    while let Some(record) = reader.next() {
        n_seqs += 1;
        let record = record.unwrap_or_else(|error| {
            panic!("'{}' contains invalid record: {error:?}", path.display())
        });
        let seq_len = record.seq_lines().fold(0, |acc, line| acc + line.len());

        if record
            .seq_lines()
            .any(|line| line.iter().any(|c| !NUCLEOTIDE_ALPAHBET.contains(c)))
        {
            alphabet = Alphabet::Protein
        }
        seq_lens.insert(seq_len);
    }

    DataSetDimensions {
        n_seqs,
        seq_len: *seq_lens.first().unwrap_or(&0)..=*seq_lens.last().unwrap_or(&0),
        alphabet,
    }
}
