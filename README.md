# zfp-sys
Raw Rust bindings to ZFP (https://github.com/LLNL/zfp).

The build supports compiling either zfp-0.5.4 or zfp-0.5.5. By default the newer v0.5.5 is used.
There are design and speed differences between these two versions.
In particular the default cache size has changed (see https://github.com/LLNL/zfp/issues/88).

To use the previous version 0.5.4 one needs to set a feature flag "0_5_4", i.e. place this in your Cargo.toml:

zfp-sys = {version = "*", features = ["0_5_4"]}