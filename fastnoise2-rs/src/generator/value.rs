use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper};

#[derive(Clone, Debug)]
pub struct Value;

impl Generator for Value {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        SafeNode(Node::from_name("Value").unwrap().into()).into()
    }
}

pub fn value() -> GeneratorWrapper<Value> {
    Value.into()
}
