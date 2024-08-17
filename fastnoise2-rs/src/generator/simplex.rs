use crate::{typed::TypedFastNoise, FastNoise};

use super::{Generator, Node};

#[derive(Clone, Copy, Debug)]
pub struct Simplex;

#[derive(Clone, Copy, Debug)]
pub struct OpenSimplex2;

#[derive(Clone, Copy, Debug)]
pub struct OpenSimplex2S;

impl Node for Simplex {
    fn build_node(&self) -> TypedFastNoise {
        TypedFastNoise(FastNoise::from_name("Simplex").unwrap())
    }
}

impl Node for OpenSimplex2 {
    fn build_node(&self) -> TypedFastNoise {
        TypedFastNoise(FastNoise::from_name("OpenSimplex2").unwrap())
    }
}

impl Node for OpenSimplex2S {
    fn build_node(&self) -> TypedFastNoise {
        TypedFastNoise(FastNoise::from_name("OpenSimplex2S").unwrap())
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
