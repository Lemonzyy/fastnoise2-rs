//! Generates `fastnoise2-rs/src/nodes` from FastNoise2 metadata.
//!
//! Run `cargo run -p fastnoise2-codegen` after updating FastNoise2, or
//! `cargo run -p fastnoise2-codegen -- --check` to check that the generated files are up to date.
use std::{collections::BTreeSet, env, fs, path::PathBuf, process::ExitCode};

mod generate;
mod metadata;

fn output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fastnoise2-rs/src/nodes")
}

/// Returns the generated files, as (file name, content).
fn generated_files() -> Vec<(String, String)> {
    let nodes = metadata::load();
    generate::Generator::new(&nodes).files()
}

/// Returns the names of the files that differ from, are missing in, or are extra in `output_dir`.
fn outdated_files(files: &[(String, String)]) -> Vec<String> {
    let output_dir = output_dir();
    let mut outdated = files
        .iter()
        .filter(|(file_name, content)| {
            fs::read_to_string(output_dir.join(file_name)).ok().as_ref() != Some(content)
        })
        .map(|(file_name, _)| file_name.clone())
        .collect::<Vec<_>>();

    let generated = files
        .iter()
        .map(|(file_name, _)| file_name.as_str())
        .collect::<BTreeSet<_>>();
    let entries = fs::read_dir(&output_dir).expect("Failed to read output directory");
    for entry in entries {
        let file_name = entry.expect("Failed to read entry").file_name();
        let file_name = file_name.to_string_lossy();
        if !generated.contains(file_name.as_ref()) {
            outdated.push(file_name.into_owned());
        }
    }

    outdated
}

fn main() -> ExitCode {
    let files = generated_files();
    let output_dir = output_dir();

    if env::args().any(|arg| arg == "--check") {
        let outdated = outdated_files(&files);
        if outdated.is_empty() {
            println!("Generated files are up to date");
            return ExitCode::SUCCESS;
        }
        eprintln!(
            "Generated files are outdated, run `cargo run -p fastnoise2-codegen`: {}",
            outdated.join(", ")
        );
        return ExitCode::FAILURE;
    }

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("Failed to remove output directory");
    }
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    for (file_name, content) in &files {
        fs::write(output_dir.join(file_name), content).expect("Failed to write generated file");
    }

    println!(
        "Generated {} files in {}",
        files.len(),
        output_dir.display()
    );
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_files_are_up_to_date() {
        let outdated = outdated_files(&generated_files());
        assert!(
            outdated.is_empty(),
            "run `cargo run -p fastnoise2-codegen`, outdated files: {outdated:?}"
        );
    }
}
