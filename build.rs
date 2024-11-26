extern crate bindgen;

#[cfg(not(feature = "cuda"))]
use cmake;

#[cfg(feature = "cuda")]
use cmake::Config;

use std::{env, path::PathBuf};

fn main() {
    let _source_dir = String::from("zfp");

    //build zfp with cmake
    let mut config = cmake::Config::new(_source_dir);
    config.define("BUILD_TESTING", "OFF");
    config.define("BUILD_UTILITIES", "OFF");

    //enable CUDA for faster compression/decompression
    #[cfg(feature = "cuda")]
    config.define("ZFP_WITH_CUDA", "ON");

    // Build a static library
    #[cfg(feature = "static")]
    {
        config.define("BUILD_SHARED_LIBS", "OFF");
        config.define("ZFP_WITH_OPENMP", "OFF");
    }

    // Enable tighter errors with proper rounding and reduced bias
    #[cfg(feature = "round-tight-error")]
    {
        config.define("ZFP_ROUNDING_MODE", "ZFP_ROUND_FIRST");
        config.define("ZFP_WITH_TIGHT_ERROR", "ON");
    }

    let zfp = config.build();

    println!("cargo:rustc-link-search=native={}/lib", zfp.display());
    println!("cargo:rustc-link-search=native={}/lib64", zfp.display());
    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=zfp");
    #[cfg(feature = "static")]
    println!("cargo:rustc-link-lib=static=zfp");

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
