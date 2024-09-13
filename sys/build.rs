use std::env;
use std::path::PathBuf;
use bindgen::Builder;

fn main() {
    // Generate the bindings
    let bindings = Builder::default()
        .header("path/to/wrapper.h")
        .generate()
        .expect("Failed to generate bindings");

    // Set the output path for the generated bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}