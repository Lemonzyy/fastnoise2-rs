// This example illustrates live editing with the FastNoise2 Node Editor: a node tree built in Rust
// is opened in the Node Editor, and every change made there is received, rebuilt and rendered.
//
// cargo run --example editor_ipc --features editor-ipc,encode -- /path/to/NodeEditor
use std::{
    env, fs,
    path::PathBuf,
    process::Stdio,
    thread,
    time::{Duration, Instant},
};

use fastnoise2::{EditorMessage, NodeEditorIpc, node_editor_command, prelude::*};
use image::{GrayImage, Luma};

const SIZE: i32 = 512;

fn main() {
    let editor_path = env::args()
        .nth(1)
        .expect("usage: editor_ipc /path/to/NodeEditor");

    let node = perlin()
        .with_feature_scale(150.0)
        .fractal_f_bm()
        .with_octaves(5)
        .build();
    render(&node);

    // Opened before starting the Node Editor, so its first selection isn't missed
    let mut ipc = NodeEditorIpc::open().expect("failed to open the Node Editor shared memory");
    let mut editor = node_editor_command(editor_path, Some(&node.encode().unwrap()), false)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to start the Node Editor");
    println!("Edit the node tree in the Node Editor, close it to exit");

    while editor.try_wait().unwrap().is_none() {
        if let Some(EditorMessage::SelectedNode(encoded)) = ipc.poll() {
            // FastNoise2 nodes can't be changed once built, the Node Editor also rebuilds the
            // whole tree from its encoded form on every change
            match Node::from_encoded_node_tree(&encoded) {
                Ok(node) => render(&node),
                Err(error) => println!("Invalid node tree {encoded}: {error}"),
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}

fn render(node: &Node) {
    let start = Instant::now();
    let mut noise = vec![0.0; (SIZE * SIZE) as usize];
    let min_max = node.gen_uniform_grid_2d(&mut noise, 0.0, 0.0, SIZE, SIZE, 1.0, 1.0, 1337);

    let image = GrayImage::from_fn(SIZE as u32, SIZE as u32, |x, y| {
        let value = noise[(y * SIZE as u32 + x) as usize];
        Luma([((value + 1.0) / 2.0 * 255.0).clamp(0.0, 255.0) as u8])
    });
    let output_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("examples_output");
    fs::create_dir_all(&output_dir).expect("Failed to create directories");
    let output_path = output_dir.join("editor_ipc.png");
    image.save(&output_path).expect("Failed to save image");

    println!(
        "{node:?} rendered in {:?} to {}: {min_max:?}",
        start.elapsed(),
        output_path.display()
    );
}
