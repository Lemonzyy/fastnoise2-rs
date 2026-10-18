//! Helpers shared by the tests of the node modules.
use super::{Node, NodeBuilder};

pub(super) fn perlin() -> Node {
    NodeBuilder::new("Perlin").unwrap().build().unwrap()
}

/// 8x8 values of a 2D grid.
pub(super) fn grid(node: &Node) -> Vec<f32> {
    let mut output = vec![0.0; 64];
    node.gen_uniform_grid_2d(&mut output, -40.0, -40.0, 8, 8, 10.0, 10.0, 1337);
    output
}
