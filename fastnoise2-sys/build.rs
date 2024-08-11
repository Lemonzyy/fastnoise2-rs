use std::env;
use std::path::PathBuf;

const LIB_DIR_ENV_KEY: &str = "FASTNOISE2_LIB_DIR";
const REL_HEADER_PATH: &str = "include/FastNoise/FastNoise_C.h";

fn main() {
    if let Ok(lib_dir) = env::var(LIB_DIR_ENV_KEY) {
        println!("cargo:rerun-if-env-changed={LIB_DIR_ENV_KEY}");
        println!("cargo:rustc-link-search=native={}", lib_dir);
        println!("cargo:rustc-link-lib=static=FastNoise");
    } else {
        #[cfg(feature = "build-from-source")]
        {
            // Build from source using CMake
            let cmake_dir = PathBuf::from("../../FastNoise2"); // TODO add env var/git submodule
            println!(
                "cargo:rerun-if-changed={}",
                cmake_dir.join(REL_HEADER_PATH).display()
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
            let out_path = cmake::Config::new(cmake_dir.clone())
                .profile("Release")
                .define("FASTNOISE2_NOISETOOL", "OFF")
                .define("FASTNOISE2_TESTS", "OFF")
                .define("BUILD_SHARED_LIBS", "OFF") // build as a static lib
                .define("CMAKE_CXX_FLAGS_RELEASE", "/MD /O2 /Ob2 /DNDEBUG")
                .build();
            let lib_path = out_path.join("lib");
            let header_path = out_path.join(REL_HEADER_PATH);

            println!("cargo:rustc-link-search=native={}", lib_path.display());
            println!("cargo:rustc-link-lib=static=FastNoise");

            let bindings = bindgen::Builder::default()
                .header(header_path.to_str().unwrap())
                .clang_arg("-xc++") // 'bool' exists in C++ but not directly in C, it's _Bool or you can use 'bool' by including 'stdbool.h'
                .generate()
                .expect("Unable to generate bindings");

            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
        }

        #[cfg(not(feature = "build-from-source"))]
        panic!("No valid build configuration found. Set {LIB_DIR_ENV_KEY} to use a precompiled library and built-in bindings or enable the build-from-source feature.");
    }
}
