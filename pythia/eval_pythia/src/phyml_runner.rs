use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

pub fn run(aln_path: &Path) -> Option<f64> {
    let aln_path = convert_to_phylip(aln_path.to_path_buf());
    match aln_path {
        Some(aln_path) => {
            println!("Running phyml analysis for {aln_path:?}");
            let start = Instant::now();
            let phyml_output = Command::new("sh")
                .arg("phyml.sh")
                .arg(aln_path)
                .output()
                .expect("Failed to start phyml");
            let runtime = start.elapsed();
            if phyml_output.status.success() {
                println!(
                    "Phyml ran successfully in {runtime:?} seconds",
                );
                Some(runtime.as_secs_f64())
            } else {
                eprintln!(
                    "Phyml failed with the following output {}",
                    String::from_utf8(phyml_output.stderr)
                        .expect("Failed to convert phyml stderr to string")
                );
                None
            }
        }
        None => None
    }
}

fn convert_to_phylip(aln_path: PathBuf) -> Option<PathBuf> {
    println!("Converting {aln_path:?} to phylip");
    let mut phy_path = aln_path.clone().to_path_buf();
    phy_path.set_extension("phy");
    let converter_output = Command::new("python")
        .arg("fasta_to_phy.py")
        .arg(&aln_path)
        .arg(&phy_path)
        .output()
        .expect("python script should be runnable");
    if converter_output.status.success() {
        Some(phy_path)
    } else {
        eprintln!(
            "Conversion to phylip failed with error {}",
            String::from_utf8(converter_output.stderr)
                .expect("Failed to convert phylip converter output to string")
        );
        None
    }
}
