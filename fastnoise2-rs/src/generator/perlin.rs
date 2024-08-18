use crate::{safe::SafeNode, Node};

use super::{Generator, TypedNode};

#[derive(Debug)]
pub struct Perlin;

impl TypedNode for Perlin {
    fn build_node(&self) -> SafeNode {
        SafeNode(Node::from_name("Perlin").unwrap())
    }
}

pub fn perlin() -> Generator<Perlin> {
    Generator(Perlin)
}
