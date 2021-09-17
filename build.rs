extern crate bindgen;

#[cfg(not(feature = "cuda"))]
use cmake;

#[cfg(feature = "cuda")]
use cmake::Config;

use std::env;
use std::path::PathBuf;

fn main() {
    let _source_dir = String::from("zfp-0.5.5");

    #[cfg(feature = "0_5_4")]
    let _source_dir = String::from("zfp-0.5.4");

    //build zfp with cmake
    #[cfg(not(feature = "cuda"))]
    let zfp = cmake::build(_source_dir);

    //enable CUDA for faster compression/decompression
    #[cfg(feature = "cuda")]
    let zfp = Config::new(_source_dir)
        .define("ZFP_WITH_CUDA", "ON")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", zfp.display());
    println!("cargo:rustc-link-search=native={}/lib64", zfp.display());
    println!("cargo:rustc-link-lib=zfp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .blocklist_type("max_align_t")
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
