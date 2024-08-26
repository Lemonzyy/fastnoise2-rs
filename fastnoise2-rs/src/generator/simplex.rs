use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper};

#[derive(Clone, Debug)]
pub struct Simplex;

#[derive(Clone, Debug)]
pub struct OpenSimplex2;

#[derive(Clone, Debug)]
pub struct OpenSimplex2S;

impl Generator for Simplex {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        SafeNode(Node::from_name("Simplex").unwrap().into()).into()
    }
}

impl Generator for OpenSimplex2 {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        SafeNode(Node::from_name("OpenSimplex2").unwrap().into()).into()
    }
}

impl Generator for OpenSimplex2S {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        SafeNode(Node::from_name("OpenSimplex2S").unwrap().into()).into()
    }
}

pub fn simplex() -> GeneratorWrapper<Simplex> {
    Simplex.into()
}

pub fn opensimplex2() -> GeneratorWrapper<OpenSimplex2> {
    OpenSimplex2.into()
}

pub fn opensimplex2s() -> GeneratorWrapper<OpenSimplex2S> {
    OpenSimplex2S.into()
}
