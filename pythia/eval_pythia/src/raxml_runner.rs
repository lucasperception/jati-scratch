use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use crate::Alphabet;

pub fn run(aln_path: &Path, alphabet: &Alphabet) -> Option<Duration> {
    println!("Running raxml for {aln_path:?}");
    let alphabet = match alphabet {
        Alphabet::Nucleotide => "nucleotide",
        Alphabet::Protein => "protein",
    };
    let start = Instant::now();
    let raxml_output = Command::new("sh")
        .arg("raxml.sh")
        .arg(aln_path)
        .arg(alphabet)
        .output()
        .expect("Failed to start raxml");
    let runtime = start.elapsed();
    if raxml_output.status.success() && raxml_output.stderr.is_empty() {
        println!("Raxml ran succesfully in {runtime:?} seconds");
        Some(runtime)
    } else {
        eprintln!("Running Raxml failed for path {aln_path:?}");
        None
    }
}
