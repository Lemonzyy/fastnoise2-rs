use crate::{safe::SafeNode, Node};

use super::{Generator, TypedNode};

#[derive(Debug)]
pub struct Value;

impl TypedNode for Value {
    fn build_node(&self) -> SafeNode {
        SafeNode(Node::from_name("Value").unwrap())
    }
}

pub fn value() -> Generator<Value> {
    Generator(Value)
}
