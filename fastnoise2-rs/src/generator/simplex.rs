use crate::FastNoise;

use super::{Generator, Node};

#[derive(Clone, Copy, Debug)]
pub struct Simplex;

impl Node for Simplex {
    fn build_node(&self) -> FastNoise {
        FastNoise::from_name("Simplex").unwrap()
    }
}

pub fn simplex() -> Generator<Simplex> {
    Generator(Simplex)
}
