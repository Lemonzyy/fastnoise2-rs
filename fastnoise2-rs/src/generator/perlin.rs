use crate::{typed::TypedFastNoise, FastNoise};

use super::{Generator, Node};

#[derive(Clone, Copy, Debug)]
pub struct Perlin;

impl Node for Perlin {
    fn build_node(&self) -> TypedFastNoise {
        TypedFastNoise(FastNoise::from_name("Perlin").unwrap())
    }
}

pub fn perlin() -> Generator<Perlin> {
    Generator(Perlin)
}
