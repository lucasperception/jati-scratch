use std::path::PathBuf;
use std::process::Command;

pub fn run(path: &PathBuf) -> f64 {
    let pythia_output = Command::new("python")
        .arg("predictor.py")
        .arg(&path)
        .output()
        .expect("python script should be runnable");
    if pythia_output.status.success() {
        let stdout = String::from_utf8(pythia_output.stdout)
            .expect("stdout of predictor is not valid utf8");
        let stdout = stdout.trim_end();
        let difficulty: f64 = match stdout.parse() {
            Ok(v) => v,
            Err(error) => panic!("failed to parse stdout '{stdout}' as a float: {error:?}"),
        };
        println!(
            "{path:?} predicted difficulty {difficulty}"
        );
        difficulty
    } else {
        eprintln!(
            "{path:?} failed to predict difficulty"
        );
        -1.0
    }
}