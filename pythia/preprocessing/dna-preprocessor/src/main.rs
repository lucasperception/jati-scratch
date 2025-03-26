use rayon::iter::{IntoParallelIterator, ParallelIterator};
use seq_io::fasta::Record;
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::exit;

const FILE_EXTENSIONS: &[&str] = &[".fas", ".fna", ".fasta", ".aln"];
const GAP_ALPHABET: [u8; 2] = [b'-', b'_'];
const NUCLEOTIDE_ALPHABET: [u8; 8] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't'];

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let [_exe, dir_arg] = &args[..] else {
        eprintln!("usage: <exe> path-to-dir-with-fasta-files-and-subdirs");
        exit(1);
    };
    let discovered_filepaths = find_fasta_files_recursive(PathBuf::from(dir_arg));
    discovered_filepaths
        .into_par_iter()
        .for_each(|fasta_file_path| {
            let mut output_path = fasta_file_path.clone();
            output_path.set_extension("processed.fasta");
            let output_file = File::create(&output_path).expect("Failed to create output file");
            let mut writer = BufWriter::new(output_file);
            let mut reader = seq_io::fasta::Reader::from_path(&fasta_file_path)
                .expect("path should point to a file");
            let mut record_counter = 0;
            while let Some(record) = reader.next() {
                let record = record.expect("Failed to read fasta record");
                write!(writer, ">{}", record.id().expect("Failed to get record ID"))
                    .expect("could not write record ID to output file");
                writeln!(writer).expect("Failed to write record to output file");
                let sequence: Vec<u8> = record
                    .seq()
                    .iter()
                    .map(|&b| {
                        let c = b;
                        if NUCLEOTIDE_ALPHABET.contains(&c) || GAP_ALPHABET.contains(&c) {
                            c
                        } else {
                            b'-'
                        }
                    })
                    .collect();
                writer
                    .write_all(&sequence)
                    .expect("Failed to write record to output file");
                writeln!(writer).expect("Failed to write record to output file");
                record_counter += 1;
            }
            writer.flush().expect("Failed to flush output file");
            println!(
                "Processed {} records for path {}",
                record_counter,
                fasta_file_path.to_string_lossy()
            );
            fs::remove_file(&fasta_file_path).expect("Failed to delete original file");
        });

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
}
