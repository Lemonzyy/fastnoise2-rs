use crate::FastNoise;

use super::{Generator, Node};

#[derive(Clone, Copy, Debug)]
pub struct Value;

impl Node for Value {
    fn build_node(&self) -> FastNoise {
        FastNoise::from_name("Value").unwrap()
    }
}

pub fn value() -> Generator<Value> {
    Generator(Value)
}
