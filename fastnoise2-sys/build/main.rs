use std::env;

const SOURCE_DIR_KEY: &str = "FASTNOISE2_SOURCE_DIR";
const LIB_DIR_KEY: &str = "FASTNOISE2_LIB_DIR";
const LIB_NAME: &str = "FastNoise";
#[cfg(feature = "build-from-source")]
const HEADER_NAME: &str = "FastNoise_C.h";

fn main() {
    println!("cargo:rerun-if-env-changed={SOURCE_DIR_KEY}");
    println!("cargo:rerun-if-env-changed={LIB_DIR_KEY}");

    if let Ok(lib_dir) = env::var(LIB_DIR_KEY) {
        println!("cargo:warning=fastnoise2: linking to the precompiled library '{LIB_NAME}' located in '{lib_dir}'");
        println!("cargo:rustc-link-search=native={lib_dir}");
        println!("cargo:rustc-link-lib=static={LIB_NAME}");
    } else {
        #[cfg(feature = "build-from-source")]
        {
            let source_dir = match env::var(SOURCE_DIR_KEY).ok() {
                Some(path) => {
                    let path = std::path::PathBuf::from(path);
                    println!(
                        "cargo:warning=fastnoise2: building from source files located in '{}'",
                        path.display()
                    );
                    path
                }
                None => {
                    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
                    std::path::Path::new(&manifest_dir)
                        .join("build")
                        .join("FastNoise2")
                }
            };

            println!(
                "cargo:rerun-if-changed={}",
                source_dir.join("include").join("FastNoise").display()
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
            let out_path = cmake::Config::new(source_dir.clone())
                .profile("Release")
                .define("FASTNOISE2_NOISETOOL", "OFF")
                .define("FASTNOISE2_TESTS", "OFF")
                .define("BUILD_SHARED_LIBS", "OFF") // build as a static lib
                .define("CMAKE_CXX_FLAGS_RELEASE", "/MD /O2 /Ob2 /DNDEBUG")
                .build();
            let lib_path = out_path.join("lib");
            let header_path = out_path.join("include").join("FastNoise").join(HEADER_NAME);

            println!("cargo:rustc-link-search=native={}", lib_path.display());
            println!("cargo:rustc-link-lib=static={LIB_NAME}");

            let bindings = bindgen::Builder::default()
                .header(header_path.to_str().unwrap())
                // 'bool' exists in C++ but not directly in C, it is named _Bool or you can use 'bool' by including 'stdbool.h'
                .clang_arg("-xc++")
                .generate()
                .expect("Unable to generate bindings");

            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
        }

        #[cfg(not(feature = "build-from-source"))]
        panic!("\
        No valid build configuration found.\n\
        To use a precompiled library and built-in bindings, set the environment variable '{LIB_DIR_KEY}' to the path of the precompiled library directory. \
        Alternatively, enable the 'build-from-source' feature to build the library and generate bindings from source.\
        ");
    }
}
