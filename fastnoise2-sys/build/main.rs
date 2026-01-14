use std::{env, path::PathBuf};

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
    println!("cargo:rerun-if-env-changed=EMSDK");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // WASM builds use pure WASM with SIMD128
    if target_arch == "wasm32" {
        build_wasm();
        return; // WASM doesn't need C++ stdlib linking
    }

    // Native builds follow existing logic
    let feature_build_from_source = env::var("CARGO_FEATURE_BUILD_FROM_SOURCE").is_ok();

    if feature_build_from_source {
        println!("cargo:warning=feature 'build-from-source' is enabled; building FastNoise2 from source");
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

    emit_std_cpp_link();
}

fn build_wasm() {
    let source_path = env::var(SOURCE_DIR_KEY).map(PathBuf::from).unwrap_or_else(|_| default_source_path());

    println!("cargo:warning=Building FastNoise2 for WASM with SIMD128 support");

    // Log the EMSDK path if set (for debugging)
    if let Ok(emsdk) = env::var("EMSDK") {
        println!("cargo:warning=EMSDK path: {}", emsdk);
    }
    println!("cargo:rerun-if-changed={}", source_path.join("include").join("FastNoise").display());

    // Get Emscripten SDK path from environment
    let emsdk_path = env::var("EMSDK").expect("EMSDK environment variable required for WASM builds. Install from https://emscripten.org");

    // Use Emscripten's CMake toolchain file - this properly configures compilers and sysroot
    let toolchain_file = format!("{}/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake", emsdk_path);

    // Build FastNoise2 for WASM as a pure static library using Emscripten toolchain
    // FastSIMD has native WASM SIMD128 support - we just need to enable it
    let mut config = cmake::Config::new(&source_path);
    config
        .profile("Release")
        .define("CMAKE_TOOLCHAIN_FILE", &toolchain_file)
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

    // Copy Utility headers that cmake doesn't install
    let src_utility = source_path.join("include").join("FastNoise").join("Utility");
    let dst_utility = out_path.join("include").join("FastNoise").join("Utility");
    if src_utility.exists() && !dst_utility.exists() {
        std::fs::create_dir_all(&dst_utility).expect("Failed to create Utility dir");
        for entry in std::fs::read_dir(&src_utility).expect("Failed to read Utility dir") {
            let entry = entry.expect("Failed to read entry");
            let dst = dst_utility.join(entry.file_name());
            std::fs::copy(entry.path(), &dst).expect("Failed to copy header");
        }
    }

    generate_bindings(out_path);
}

fn build_from_source() {
    let source_path = env::var(SOURCE_DIR_KEY).map(PathBuf::from).unwrap_or_else(|_| default_source_path());

    println!("cargo:warning=building from source files located in '{}'", source_path.display());
    println!("cargo:rerun-if-changed={}", source_path.join("include").join("FastNoise").display());

    // Pre-create pdb-files directory structure to prevent CMake install failure on Windows
    // FastNoise2's CMakeLists.txt tries to install PDB files that may not exist in Release builds
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let pdb_dir = out_dir.join("build").join("pdb-files").join("Release");
    std::fs::create_dir_all(&pdb_dir).ok();

    let mut config = cmake::Config::new(&source_path);
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

    // Copy Utility headers that cmake doesn't install
    let src_utility = source_path.join("include").join("FastNoise").join("Utility");
    let dst_utility = out_path.join("include").join("FastNoise").join("Utility");
    if src_utility.exists() && !dst_utility.exists() {
        std::fs::create_dir_all(&dst_utility).expect("Failed to create Utility dir");
        for entry in std::fs::read_dir(&src_utility).expect("Failed to read Utility dir") {
            let entry = entry.expect("Failed to read entry");
            let dst = dst_utility.join(entry.file_name());
            std::fs::copy(entry.path(), &dst).expect("Failed to copy header");
        }
    }

    generate_bindings(out_path);
}

fn generate_bindings(source_path: PathBuf) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");

    // Check for cached bindings first
    if let Ok(cache_dir) = env::var(BINDINGS_CACHE_KEY) {
        let cached_bindings = PathBuf::from(&cache_dir).join("bindings.rs");
        if cached_bindings.exists() {
            println!("cargo:warning=using cached bindings from '{}'", cached_bindings.display());
            std::fs::copy(&cached_bindings, &bindings_path).expect("Failed to copy cached bindings");
            return;
        }
    }

    println!(
        "cargo:warning=generating Rust bindings for FastNoise2 (this is slow, set \
     FASTNOISE2_BINDINGS_DIR to cache)"
    );

    let include_path = source_path.join("include").join("FastNoise");
    let header_path = include_path.join(HEADER_NAME);

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
        println!("cargo:warning=cross-compiling to WASM, using host target '{}' for bindgen", host);
        builder = builder.clang_arg(format!("--target={}", host));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings.write_to_file(&bindings_path).expect("Couldn't write bindings!");

    println!("cargo:warning=bindings generated successfully and written to '{}'", bindings_path.display());

    // Save to cache if dir is set
    if let Ok(cache_dir) = env::var(BINDINGS_CACHE_KEY) {
        let cache_path = PathBuf::from(&cache_dir);
        std::fs::create_dir_all(&cache_path).ok();
        let cached_bindings = cache_path.join("bindings.rs");
        std::fs::copy(&bindings_path, &cached_bindings).ok();
        println!("cargo:warning=bindings cached to '{}'", cached_bindings.display());
    }
}

fn default_source_path() -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("build");
    path.push("FastNoise2");
    path
}

fn emit_std_cpp_link() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        ("macos" | "freebsd", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        ("windows", "msvc") => {} // MSVC links C++ stdlib automatically
        _ => println!("cargo:warning=Unknown target for C++ stdlib linking"),
    }
}
