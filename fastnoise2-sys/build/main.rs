use std::{
    env, fs,
    path::{Path, PathBuf},
};

use cmake::Config;

const SOURCE_DIR_KEY: &str = "FASTNOISE2_SOURCE_DIR";
const LIB_DIR_KEY: &str = "FASTNOISE2_LIB_DIR";
const BINDINGS_CACHE_KEY: &str = "FASTNOISE2_BINDINGS_DIR";
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
    println!("cargo:rerun-if-env-changed={BINDINGS_CACHE_KEY}");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // WASM builds use pure WASM with SIMD128
    if target_arch == "wasm32" {
        build_wasm();
        emit_std_cpp_link();
        return;
    }

    // Native builds follow existing logic
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
        println!("cargo:rerun-if-changed={lib_dir}");

        let source_path = source_path();
        println!(
            "cargo:rerun-if-changed={}",
            source_path.join("include").join("FastNoise").display()
        );
        check_precompiled_library(Path::new(&lib_dir), &source_path);
        generate_bindings(source_path);
    } else {
        println!("cargo:warning={LIB_DIR_KEY} is not set; falling back to building from source");
        build_from_source();
    }

    emit_std_cpp_link();
}

fn build_wasm() {
    let source_path = source_path();

    println!("cargo:warning=Building FastNoise2 for WASM with SIMD128 support");

    // Watch the whole source tree, not only headers, so C++ and CMake changes rebuild the library
    println!("cargo:rerun-if-changed={}", source_path.display());

    // Build FastNoise2 for WASM as a pure static library. cmake-rs runs CMake through
    // emcmake for Emscripten targets, which sets up the Emscripten toolchain, so only
    // Emscripten in PATH is needed (e.g. from emsdk_env.sh or a Nix shell)
    // FastSIMD has native WASM SIMD128 support - we just need to enable it
    let mut config = new_cmake_config(&source_path);
    config
        .profile("Release")
        .define("FASTNOISE2_TOOLS", "OFF")
        .define("FASTNOISE2_TESTS", "OFF")
        .define("FASTNOISE2_UTILITY", "OFF") // Disable utility to avoid Corrade dependency
        .define("BUILD_SHARED_LIBS", "OFF");

    // Enable WASM SIMD128 only (no threading/atomics for compatibility with simple WASM demos)
    // NOTE: If Rust is built with --shared-memory, FastNoise2 also needs atomics (-pthread)
    // For now, keep it simple and let individual projects add atomics if needed
    let wasm_flags = "-msimd128";
    config.define("CMAKE_C_FLAGS", wasm_flags);
    config.define("CMAKE_CXX_FLAGS", wasm_flags);

    let out_path = config.build();
    let lib_path = out_path.join("lib");
    let lib64_path = out_path.join("lib64");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-search=native={}", lib64_path.display());
    println!("cargo:rustc-link-lib=static={LIB_NAME}");

    generate_bindings(out_path);
}

fn build_from_source() {
    let source_path = source_path();

    println!(
        "cargo:warning=building from source files located in '{}'",
        source_path.display()
    );
    // Watch the whole source tree, not only headers, so C++ and CMake changes rebuild the library
    println!("cargo:rerun-if-changed={}", source_path.display());

    // Pre-create pdb-files directory structure to prevent CMake install failure on Windows
    // FastNoise2's CMakeLists.txt tries to install PDB files that may not exist in Release builds
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let pdb_dir = out_dir.join("build").join("pdb-files").join("Release");
    fs::create_dir_all(&pdb_dir).ok();

    let mut config = new_cmake_config(&source_path);
    config
        .profile("Release")
        .define("FASTNOISE2_TOOLS", "OFF")
        .define("FASTNOISE2_TESTS", "OFF")
        .define("FASTNOISE2_UTILITY", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF");

    // https://github.com/rust-lang/cmake-rs/issues/198:
    // cmake-rs add default arguments such as CMAKE_CXX_FLAGS_RELEASE to the build
    // command. Removing these would automatically add Release profile args to
    // allow for better execution performance. FastNoise2 wiki steps for compiling the library (https://github.com/Auburn/FastNoise2/wiki/Compiling-FastNoise2):
    // 1. cmake -S . -B build -D FASTNOISE2_NOISETOOL=OFF -D FASTNOISE2_TESTS=OFF -D
    //    BUILD_SHARED_LIBS=OFF
    // 2. cmake --build build --config Release
    // This give us optimized build (with MSVC):
    // -> build/CMakeCache.txt: CMAKE_CXX_FLAGS_RELEASE:STRING=/MD /O2 /Ob2 /DNDEBUG
    // Whereas when using cmake-rs:
    // -> build/CMakeCache.txt: CMAKE_CXX_FLAGS_RELEASE:STRING= -nologo -MD -Brepro
    // Replace default arguments with those from the FastNoise2 manual build

    // Set optimization flags based on the target
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    let cmake_cxx_flags_release = match (target_os.as_str(), target_env.as_str()) {
        ("windows", "msvc") => "/MD /O2 /Ob2 /DNDEBUG",
        // For GCC/Clang (Linux, macOS, MinGW)
        _ => "-O3 -DNDEBUG",
    };

    println!(
        "cargo:warning=CARGO_CFG_TARGET_OS='{target_os}' and CARGO_CFG_TARGET_ENV='{target_env}' => \
     CMAKE_CXX_FLAGS_RELEASE='{cmake_cxx_flags_release}'"
    );
    config.define("CMAKE_CXX_FLAGS_RELEASE", cmake_cxx_flags_release);

    let out_path = config.build();
    let lib_path = out_path.join("lib");
    let lib64_path = out_path.join("lib64");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-search=native={}", lib64_path.display());
    println!("cargo:rustc-link-lib=static={LIB_NAME}");

    generate_bindings(out_path);
}

/// Checks that the precompiled library exports every function of the C header, to reject
/// a library built from another FastNoise2 version instead of linking mismatched functions.
fn check_precompiled_library(lib_dir: &Path, source_path: &Path) {
    let lib_path = [format!("lib{LIB_NAME}.a"), format!("{LIB_NAME}.lib")]
        .iter()
        .map(|file_name| lib_dir.join(file_name))
        .find(|path| path.exists())
        .unwrap_or_else(|| {
            panic!(
                "no {LIB_NAME} static library found in '{}'",
                lib_dir.display()
            )
        });

    let lib = fs::read(&lib_path).expect("Failed to read precompiled library");

    let header_path = source_path
        .join("include")
        .join("FastNoise")
        .join(HEADER_NAME);
    let header = fs::read_to_string(&header_path).expect("Failed to read FastNoise C header");

    let missing = header
        .split("FASTNOISE_API")
        .skip(1)
        .filter_map(|declaration| declaration.split('(').next()?.split_whitespace().last())
        .map(|name| name.trim_start_matches('*'))
        .filter(|name| {
            !lib.windows(name.len())
                .any(|window| window == name.as_bytes())
        })
        .collect::<Vec<_>>();

    assert!(
        missing.is_empty(),
        "'{}' does not match the FastNoise2 version of '{}', missing functions: {}",
        lib_path.display(),
        header_path.display(),
        missing.join(", ")
    );
}

fn generate_bindings(source_path: PathBuf) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");

    let include_path = source_path.join("include").join("FastNoise");
    let header_path = include_path.join(HEADER_NAME);

    let header = fs::read(&header_path).expect("Failed to read FastNoise C header");

    // Cached bindings are stored per crate version along with the header they were
    // generated from, and only reused if that header is identical to the current one
    let cache_path = env::var(BINDINGS_CACHE_KEY)
        .ok()
        .map(|cache_dir| PathBuf::from(cache_dir).join(env!("CARGO_PKG_VERSION")));

    // Check for cached bindings first
    if let Some(cache_path) = &cache_path {
        let cached_bindings = cache_path.join("bindings.rs");
        let cached_header = fs::read(cache_path.join(HEADER_NAME)).ok();
        if cached_bindings.exists() && cached_header.as_ref() == Some(&header) {
            println!(
                "cargo:warning=using cached bindings from '{}'",
                cached_bindings.display()
            );
            fs::copy(&cached_bindings, &bindings_path).expect("Failed to copy cached bindings");
            return;
        }
    }

    println!(
        "cargo:warning=generating Rust bindings for FastNoise2 (this is slow, set \
     FASTNOISE2_BINDINGS_DIR to cache)"
    );

    // FastNoise C API bindings are target-agnostic (pure extern "C" declarations
    // with only primitive types like c_int, c_void, f32, bool).
    // We explicitly generate for the HOST system, not the cross-compile target,
    // because bindgen/libclang fails when targeting WASM (produces empty output).
    // This is safe because the C ABI for these declarations is identical across platforms.
    let mut builder = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .clang_arg(format!("-I{}", include_path.to_str().unwrap()))
        .clang_arg("-xc++")
        .clang_arg("-fno-exceptions");

    // When cross-compiling (e.g., to WASM), force bindgen to use host target
    // instead of the cross-compile target. This works because the FastNoise C API
    // only uses portable types (c_int, c_uint, c_void*, f32, bool).
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch == "wasm32" {
        // Use host triple for parsing - the bindings are ABI-compatible
        let host = env::var("HOST").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".to_string());
        println!(
            "cargo:warning=cross-compiling to WASM, using host target '{}' for bindgen",
            host
        );
        builder = builder.clang_arg(format!("--target={}", host));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    println!(
        "cargo:warning=bindings generated successfully and written to '{}'",
        bindings_path.display()
    );

    // Save to cache if dir is set
    if let Some(cache_path) = &cache_path {
        fs::create_dir_all(cache_path).ok();
        let cached_bindings = cache_path.join("bindings.rs");
        fs::copy(&bindings_path, &cached_bindings).ok();
        fs::write(cache_path.join(HEADER_NAME), &header).ok();
        println!(
            "cargo:warning=bindings cached to '{}'",
            cached_bindings.display()
        );
    }
}

/// Creates the CMake config, using the bundled FastSIMD with the bundled FastNoise2 so the
/// build doesn't download it
fn new_cmake_config(source_path: &Path) -> Config {
    let mut config = Config::new(source_path);

    if env::var(SOURCE_DIR_KEY).is_err() {
        let fastsimd_path = default_fastsimd_path();
        println!("cargo:rerun-if-changed={}", fastsimd_path.display());
        config.define("CPM_FastSIMD_SOURCE", &fastsimd_path);
    }

    config
}

fn source_path() -> PathBuf {
    env::var(SOURCE_DIR_KEY)
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_source_path())
}

fn default_source_path() -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("build");
    path.push("FastNoise2");
    path
}

fn default_fastsimd_path() -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("build");
    path.push("FastSIMD");
    path
}

fn emit_std_cpp_link() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux" | "netbsd", _) | ("windows", "gnu") => {
            println!("cargo:rustc-link-lib=dylib=stdc++")
        }
        ("macos" | "ios" | "tvos" | "watchos" | "visionos" | "freebsd" | "openbsd", _)
        | ("windows", "gnullvm") => println!("cargo:rustc-link-lib=dylib=c++"),
        ("android", _) => println!("cargo:rustc-link-lib=dylib=c++_shared"),
        ("emscripten", _) => {
            println!("cargo:rustc-link-lib=c++");
            println!("cargo:rustc-link-lib=c++abi");
        }
        ("windows", "msvc") => {} // MSVC links C++ stdlib automatically
        _ => println!("cargo:warning=Unknown target for C++ stdlib linking"),
    }
}
