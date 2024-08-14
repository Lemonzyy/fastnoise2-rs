use std::{env, path::PathBuf};

const SOURCE_DIR_KEY: &str = "FASTNOISE2_SOURCE_DIR";
const LIB_DIR_KEY: &str = "FASTNOISE2_LIB_DIR";
const LIB_NAME: &str = "FastNoise";
const HEADER_NAME: &str = "FastNoise_C.h";

fn main() {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=docs.rs compilation detected, only bindings will be generated");
        generate_bindings(default_source_path());
        return;
    }

    println!("cargo:rerun-if-env-changed={SOURCE_DIR_KEY}");
    println!("cargo:rerun-if-env-changed={LIB_DIR_KEY}");

    let feature_build_from_source = env::var("CARGO_FEATURE_BUILD_FROM_SOURCE").is_ok();

    if feature_build_from_source {
        println!(
            "cargo:warning=feature 'build-from-source' is enabled; building FastNoise2 from source"
        );
        build_from_source();
    } else if let Ok(lib_dir) = env::var(LIB_DIR_KEY) {
        println!("cargo:warning=using precompiled library located in '{lib_dir}'");
        println!("cargo:rustc-link-search=native={lib_dir}");
        println!("cargo:rustc-link-lib=static={LIB_NAME}");

        generate_bindings(default_source_path());
    } else {
        println!("cargo:warning={LIB_DIR_KEY} is not set; falling back to building from source");
        build_from_source();
    }
}

fn build_from_source() {
    let source_path = env::var(SOURCE_DIR_KEY)
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_source_path());

    println!(
        "cargo:warning=building from source files located in '{}'",
        source_path.display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        source_path.join("include").join("FastNoise").display()
    );

    // https://github.com/rust-lang/cmake-rs/issues/198:
    // cmake-rs add default arguments such as CMAKE_CXX_FLAGS_RELEASE to the build command.
    // Removing these would automatically add Release profile args to allow for better execution performance.
    // FastNoise2 wiki steps for compiling the library (https://github.com/Auburn/FastNoise2/wiki/Compiling-FastNoise2):
    // 1. cmake -S . -B build -D FASTNOISE2_NOISETOOL=OFF -D FASTNOISE2_TESTS=OFF -D BUILD_SHARED_LIBS=OFF
    // 2. cmake --build build --config Release
    // This give us optimized build:
    // -> build/CMakeCache.txt: CMAKE_CXX_FLAGS_RELEASE:STRING=/MD /O2 /Ob2 /DNDEBUG
    // Whereas when using cmake-rs:
    // -> build/CMakeCache.txt: CMAKE_CXX_FLAGS_RELEASE:STRING= -nologo -MD -Brepro
    // Replace default arguments with those from the FastNoise2 manual build
    let out_path = cmake::Config::new(&source_path)
        .profile("Release")
        .define("FASTNOISE2_NOISETOOL", "OFF")
        .define("FASTNOISE2_TESTS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF") // build as a static lib
        .define("CMAKE_CXX_FLAGS_RELEASE", "/MD /O2 /Ob2 /DNDEBUG")
        .build();
    let lib_path = out_path.join("lib");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static={LIB_NAME}");

    generate_bindings(out_path);
}

fn generate_bindings(source_path: PathBuf) {
    println!("cargo:warning=generating Rust bindings for FastNoise2");

    let header_path = source_path
        .join("include")
        .join("FastNoise")
        .join(HEADER_NAME);
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        // 'bool' exists in C++ but not directly in C, it is named _Bool or you can use 'bool' by including 'stdbool.h'
        .clang_arg("-xc++")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    println!(
        "cargo:warning=bindings generated successfully and written to '{}'",
        bindings_path.display()
    );
}

fn default_source_path() -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("build");
    path.push("FastNoise2");
    path
}
