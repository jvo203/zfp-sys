extern crate bindgen;

use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    //build zfp with cmake
    let zfp = cmake::build("zfp-0.5.4");
    println!("cargo:rustc-link-search=native={}/lib", zfp.display());
    println!("cargo:rustc-link-search=native={}/lib64", zfp.display());
    println!("cargo:rustc-link-lib=zfp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .blacklist_type("max_align_t")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // add the location of zfp header files
        .clang_arg("-I")
        .clang_arg(format!("{}/include", zfp.display()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
