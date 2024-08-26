use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper};

#[derive(Clone, Debug)]
pub struct Perlin;

impl Generator for Perlin {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        SafeNode(Node::from_name("Perlin").unwrap().into()).into()
    }
}

pub fn perlin() -> GeneratorWrapper<Perlin> {
    Perlin.into()
}
