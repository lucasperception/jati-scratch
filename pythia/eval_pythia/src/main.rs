use std::{
    collections::{BTreeSet, VecDeque},
    ops::RangeInclusive,
    path::{Path, PathBuf},
    process::exit,
    time::Duration,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

mod phyml_runner;
mod pythia_runner;
mod raxml_runner;

#[derive(Clone, Debug)]
struct DataSet {
    path: PathBuf,
    difficulty: Option<f64>,
    ext_aligners: Option<ExternalAlignerResults>,
    dimensions: DataSetDimensions,
}

impl DataSet {
    fn into_flat(self) -> FlatDataset {
        FlatDataset {
            path: self.path.to_string_lossy().to_string(),
            difficulty: self.difficulty,
            n_seqs: self.dimensions.n_seqs,
            seq_len_min: *self.dimensions.seq_len.start(),
            seq_len_max: *self.dimensions.seq_len.end(),
            alphabet: self.dimensions.alphabet,
            phyml_runtime_ms: self
                .ext_aligners
                .and_then(|t| t.phyml_runtime.map(|duration| duration.as_millis())),
            raxml_runtime_ms: self
                .ext_aligners
                .and_then(|t| t.raxml_runtime.map(|duration| duration.as_millis())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FlatDataset {
    path: String,
    difficulty: Option<f64>,
    n_seqs: usize,
    seq_len_min: usize,
    seq_len_max: usize,
    alphabet: Alphabet,
    raxml_runtime_ms: Option<u128>,
    phyml_runtime_ms: Option<u128>,
}

const FILE_EXTENSIONS: &[&str] = &[".fas", ".fna", ".fasta", ".aln"];

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let [_exe, dir_arg] = &args[..] else {
        eprintln!("usage: <exe> path-to-dir-with-fasta-files-and-subdirs");
        exit(1);
    };
    let do_predict_difficulty = false;
    let do_run_ext_aligners = false;

    if !do_predict_difficulty {
        println!("skipping difficulty prediction");
    }
    if !do_run_ext_aligners {
        println!("skipping external aligners");
    }

    let discovered_filepaths = find_fasta_files_recursive(PathBuf::from(dir_arg));
    let mut discovered_datasets: Vec<_> = discovered_filepaths
        .into_par_iter()
        .map(|fasta_file_path| {
            let dimensions = read_n_seqs_and_seq_len(&fasta_file_path);
            let difficulty = if do_predict_difficulty {
                let difficulty = pythia_runner::run(&fasta_file_path);
                if difficulty.is_none() {
                    eprintln!("failed to estimate difficulty for the dataset {fasta_file_path:?}");
                }
                difficulty
            } else {
                None
            };
            let captured = DataSet {
                path: fasta_file_path,
                difficulty,
                dimensions,
                ext_aligners: None,
            };
            println!("evaluated dataset: {captured:?}");
            captured
        })
        .collect();

    // can not be parallelized because it would destroy runtime measurements
    if do_run_ext_aligners {
        println!("running external aligners");
        discovered_datasets = discovered_datasets
            .into_iter()
            .map(|data_set| {
                if data_set.difficulty.is_some() {
                    DataSet {
                        ext_aligners: Some(run_external_aligners(&data_set.path)),
                        ..data_set
                    }
                } else {
                    data_set
                }
            })
            .collect();
    }
    println!(
        "successfully evaluated a total of {} datasets",
        discovered_datasets.len()
    );

    println!("most difficult datasets sorted in order");

    discovered_datasets.sort_unstable_by(|left, right| {
        right
            .difficulty
            .unwrap_or(0.)
            .partial_cmp(&left.difficulty.unwrap_or(0.))
            .expect("difficulty should not have NaN's")
    });

    println!("{:?}", &discovered_datasets);
    let mut csv_writer =
        csv::Writer::from_path("eval.csv").expect("failed to open csv output file");
    discovered_datasets.into_iter().for_each(|dataset| {
        csv_writer
            .serialize(dataset.into_flat())
            .expect("failed to write dataset to csv")
    });
    csv_writer.flush().expect("failed to flush output csv");
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
            .any(|line| line.iter().any(|c| !Alphabet::is_possible_dna_char(*c)))
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

const GAP_ALPHABET: [u8; 2] = [b'-', b'_'];
const NUCLEOTIDE_ALPHABET: [u8; 8] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't'];

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Alphabet {
    Nucleotide,
    Protein,
}

impl Alphabet {
    /// TODO: some sequences like eg in WickD3b also contain 'N', I'm not yet sure
    /// what to make of it
    fn is_possible_dna_char(char: u8) -> bool {
        GAP_ALPHABET.contains(&char) || NUCLEOTIDE_ALPHABET.contains(&char)
    }
}
#[derive(Debug, Clone)]
struct DataSetDimensions {
    n_seqs: usize,
    seq_len: RangeInclusive<usize>,
    alphabet: Alphabet,
}

#[derive(Debug, Clone, Copy)]
struct ExternalAlignerResults {
    raxml_runtime: Option<Duration>,
    phyml_runtime: Option<Duration>,
}
fn run_external_aligners(fasta_file: &Path) -> ExternalAlignerResults {
    ExternalAlignerResults {
        raxml_runtime: raxml_runner::run(fasta_file),
        phyml_runtime: phyml_runner::run(fasta_file),
    }
}

fn find_fasta_files_recursive(root: PathBuf) -> Vec<PathBuf> {
    let mut dir_queue = VecDeque::from([root]);
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
    discovered_filepaths
}
