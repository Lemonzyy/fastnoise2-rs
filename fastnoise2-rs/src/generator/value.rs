use crate::{typed::TypedFastNoise, FastNoise};

use super::{Generator, Node};

#[derive(Clone, Copy, Debug)]
pub struct Value;

impl Node for Value {
    fn build_node(&self) -> TypedFastNoise {
        TypedFastNoise(FastNoise::from_name("Value").unwrap())
    }
}

pub fn value() -> Generator<Value> {
    Generator(Value)
}
