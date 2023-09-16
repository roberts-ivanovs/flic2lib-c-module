use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=../../flic-source");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    gen_file("flic2.h");
}

fn gen_file(path: &str) {
    println!("cargo:rerun-if-changed=../../flic-source/{path}");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header(format!("../../flic-source/{path}").as_str())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let output_file = path.replace(".h", ".rs");
    bindings
        .write_to_file(out_path.join(output_file))
        .expect("Couldn't write bindings!");
}
