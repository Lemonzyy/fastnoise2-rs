// This example illustrates the use of "Node::encode" to export a node tree built in Rust, in the
// format of the Node Editor: paste the printed string in the Node Editor to edit the tree there.
use fastnoise2::prelude::*;

const SIZE: i32 = 256;

fn create_node() -> Node {
    // Used twice below: a shared node is encoded once, then referenced
    let perlin = perlin().with_feature_scale(150.0).build();

    let fbm = perlin.fractal_f_bm().with_octaves(5).with_gain(0.4);
    let ridges = 1.0 - perlin.fractal_ridged().with_octaves(3).abs();

    (fbm + ridges * 0.5).build()
}

fn grid(node: &Node) -> Vec<f32> {
    let mut noise = vec![0.0; (SIZE * SIZE) as usize];
    node.gen_uniform_grid_2d(&mut noise, 0.0, 0.0, SIZE, SIZE, 1.0, 1.0, 1337);
    noise
}

fn main() {
    let node = create_node();

    let encoded = node.encode().unwrap();
    println!("Encoded node tree: {encoded}");

    // FastNoise2 decodes the string into the same tree, generating the same noise
    let decoded = Node::from_encoded_node_tree(&encoded).unwrap();
    assert_eq!(grid(&node), grid(&decoded));
    println!("The decoded node tree generates the same noise");
}
