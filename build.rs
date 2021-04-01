use cmake::Config;
use std::env;

fn main() {
    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    let dst = Config::new("CSFML")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .build();
    println!("cargo:warning={}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-flags=-lstdc++");
    println!("cargo:rustc-link-lib=static=csfml-system");
    if env::var("CARGO_FEATURE_AUDIO").is_ok() {
        println!("cargo:rustc-link-lib=static=csfml-audio");
    }
    if env::var("CARGO_FEATURE_WINDOW").is_ok() {
        println!("cargo:rustc-link-lib=static=csfml-window");
    }
    if env::var("CARGO_FEATURE_GRAPHICS").is_ok() {
        println!("cargo:rustc-link-lib=static=csfml-graphics");
    }
}
