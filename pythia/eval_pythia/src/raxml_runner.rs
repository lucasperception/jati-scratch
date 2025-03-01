use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

pub fn run(aln_path: &PathBuf) -> f64 {
    let start = Instant::now();
    let raxml_output = Command::new("sh")
        .arg("raxml.sh")
        .arg(aln_path)
        .output()
        .expect("Failed to start raxml");
    let runtime = start.elapsed();
    if raxml_output.status.success()
        && raxml_output.stderr.is_empty()
    {
        println!("Raxml ran succesfully in {} seconds", runtime.as_secs_f64());
        runtime.as_secs_f64()
    } else {
        -1.0
    }
}
