use std::path::Path;
use std::process::Command;

pub fn run(path: &Path) -> Option<f64> {
    let pythia_output = Command::new("python")
        .arg("predictor.py")
        .arg(path)
        .output()
        .expect("python script should be runnable");
    if pythia_output.status.success() {
        let stdout =
            String::from_utf8(pythia_output.stdout).expect("stdout of predictor is not valid utf8");
        let stdout = stdout.trim_end();
        let difficulty: f64 = stdout
            .parse()
            .unwrap_or_else(|e| panic!("failed to parse stdout '{stdout}' as a float: {e:?}"));
        println!("{path:?} predicted difficulty {difficulty}");
        Some(difficulty)
    } else {
        eprintln!("{path:?} failed to predict difficulty");
        None
    }
}
