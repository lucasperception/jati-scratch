use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

pub fn run(aln_path: &Path) -> Option<Duration> {
    let start = Instant::now();
    let raxml_output = Command::new("sh")
        .arg("raxml.sh")
        .arg(aln_path)
        .output()
        .expect("Failed to start raxml");
    let runtime = start.elapsed();
    if raxml_output.status.success() && raxml_output.stderr.is_empty() {
        println!("Raxml ran succesfully in {runtime:?} seconds");
        Some(runtime)
    } else {
        None
    }
}
