use crate::{safe::SafeNode, Node};

use super::{Generator, TypedNode};

#[derive(Debug)]
pub struct Simplex;

#[derive(Debug)]
pub struct OpenSimplex2;

#[derive(Debug)]
pub struct OpenSimplex2S;

impl TypedNode for Simplex {
    fn build_node(&self) -> SafeNode {
        SafeNode(Node::from_name("Simplex").unwrap())
    }
}

impl TypedNode for OpenSimplex2 {
    fn build_node(&self) -> SafeNode {
        SafeNode(Node::from_name("OpenSimplex2").unwrap())
    }
}

impl TypedNode for OpenSimplex2S {
    fn build_node(&self) -> SafeNode {
        SafeNode(Node::from_name("OpenSimplex2S").unwrap())
    }
}

pub fn simplex() -> Generator<Simplex> {
    Generator(Simplex)
}

pub fn opensimplex2() -> Generator<OpenSimplex2> {
    Generator(OpenSimplex2)
}

pub fn opensimplex2s() -> Generator<OpenSimplex2S> {
    Generator(OpenSimplex2S)
}
