use std::{env, path::PathBuf};

use cmake::Config;

fn main() {
    let out = Config::new("./CPythia/").build();

    println!("cargo:rustc-link-search={}/build/src/corax", out.display());
    println!("cargo:rustc-link-lib=static=coraxlib_difficulty_prediction_lib");

    let bindings = bindgen::Builder::default()
        .header("./CPythia/src/corax/difficulty.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("corax_msa_compute_features")
        .allowlist_function("corax_msa_predict_difficulty")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
