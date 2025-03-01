use std::{env, path::PathBuf};

use cmake::Config;

fn main() {
    let _out_path = Config::new("./coraxlib/")
        .no_build_target(true)
        .define("CORAX_BUILD_DIFFICULTY_PREDICTION", "On")
        .build();
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-link-search={manifest_dir}/coraxlib/bin");
    println!("cargo:rustc-link-lib=static=corax");
    println!("cargo:rustc-link-lib=static=coraxlib_difficulty_prediction_lib");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I./coraxlib/src/")
        .header("./coraxlib/src/corax/corax.h")
        .header("./coraxlib/lib/difficulty_prediction/src/corax/difficulty.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_type("corax_split_t")
        .allowlist_type("corax_msa_t")
        .allowlist_type("corax_utree_t")
        .allowlist_function("corax_utree_create_parsimony")
        .allowlist_function("corax_utree_split_create")
        .allowlist_function("corax_utree_destroy")
        .allowlist_function("corax_utree_split_rf_distance")
        .allowlist_function("corax_msa_destroy")
        .allowlist_function("corax_utree_split_destroy")
        .allowlist_function("corax_(phylip|fasta)_load")
        .allowlist_item("CORAX_(TRUE|FALSE)")
        .allowlist_item("corax_map_(nt|aa)")
        .allowlist_function("corax_msa_compute_features")
        .allowlist_function("corax_msa_predict_difficulty")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
