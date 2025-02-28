use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=corax");

    let bindings = bindgen::Builder::default()
        .header("./wrapper.h")
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
        .allowlist_item("corax_map_nt")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
