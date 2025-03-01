use std::{
    collections::HashSet,
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

    for fasta_file_path in fasta_file_paths {
        let (n_seqs, seq_len) = read_n_seqs_and_seq_len(&fasta_file_path);
        println!("Evaluating {fasta_file_path:?} with dimensions ({n_seqs},{seq_len})");
        let difficulty = pythia_runner::run(&fasta_file_path);
        let raxml_runtime = raxml_runner::run(&fasta_file_path);
        let phyml_runtime = phyml_runner::run(&fasta_file_path);
        evaluated_datasets.push(DataSet {
            path: fasta_file_path,
            n_seqs,
            seq_len,
            difficulty,
            raxml_runtime,
            phyml_runtime,
        })
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
}

fn read_n_seqs_and_seq_len(path: &Path) -> (usize, usize) {
    let mut reader = seq_io::fasta::Reader::from_path(path).expect("path should point to a file");

    let mut n_seqs = 0;
    let mut seq_lens = HashSet::new();
    while let Some(record) = reader.next() {
        n_seqs += 1;
        let record = record.expect("file contains invalid record");
        let seq_len = record.seq_lines().fold(0, |acc, line| acc + line.len());
        seq_lens.insert(seq_len);
    }
    if seq_lens.len() > 1 {
        println!("non-unique sequence lengths in {path:?}");
    }

    (n_seqs, seq_lens.into_iter().next().unwrap_or(0))
}
