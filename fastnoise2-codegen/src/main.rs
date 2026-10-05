//! Generates `fastnoise2-rs/src/nodes` from FastNoise2 metadata.
//!
//! Run `cargo run -p fastnoise2-codegen` after updating FastNoise2.
use std::{fs, path::PathBuf};

mod generate;
mod metadata;

fn output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fastnoise2-rs/src/nodes")
}

fn main() {
    let nodes = metadata::load();
    let files = generate::Generator::new(&nodes).files();

    let output_dir = output_dir();
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    for (file_name, content) in &files {
        fs::write(output_dir.join(file_name), content).expect("Failed to write generated file");
    }

    println!(
        "Generated {} files for {} nodes in {}",
        files.len(),
        nodes.len(),
        output_dir.display()
    );
}
