//! Values of node members: hybrids and the values set by name.
use super::{Generator, Node};
use crate::MemberType;

/// Value of a hybrid member: a constant or a node.
#[derive(Clone, Debug)]
pub enum Hybrid {
    Value(f32),
    Node(Node),
}

impl From<f32> for Hybrid {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

impl<G: Generator> From<G> for Hybrid {
    fn from(generator: G) -> Self {
        Self::Node(generator.build())
    }
}

/// Value of any member, set by name with [`NodeBuilder::set`](crate::NodeBuilder::set).
#[derive(Clone, Debug)]
pub enum MemberValue {
    Float(f32),
    Int(i32),
    /// Enum value name, compared ignoring case and spaces.
    Enum(String),
    Node(Node),
}

impl MemberValue {
    pub(super) fn member_type(&self) -> MemberType {
        match self {
            Self::Float(_) => MemberType::Float,
            Self::Int(_) => MemberType::Int,
            Self::Enum(_) => MemberType::Enum,
            Self::Node(_) => MemberType::NodeLookup,
        }
    }
}

impl From<f32> for MemberValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i32> for MemberValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<&str> for MemberValue {
    fn from(value: &str) -> Self {
        Self::Enum(value.to_string())
    }
}

impl From<String> for MemberValue {
    fn from(value: String) -> Self {
        Self::Enum(value)
    }
}

impl From<Hybrid> for MemberValue {
    fn from(hybrid: Hybrid) -> Self {
        match hybrid {
            Hybrid::Value(value) => Self::Float(value),
            Hybrid::Node(node) => Self::Node(node),
        }
    }
}

impl From<&Hybrid> for MemberValue {
    fn from(hybrid: &Hybrid) -> Self {
        match hybrid {
            Hybrid::Value(value) => Self::Float(*value),
            Hybrid::Node(node) => Self::Node(node.clone()),
        }
    }
}

impl<G: Generator> From<G> for MemberValue {
    fn from(generator: G) -> Self {
        Self::Node(generator.build())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{super::test_utils::perlin, *};

    #[test]
    fn test_member_value_from_hybrid_reference() {
        let hybrid = Hybrid::from(perlin());
        let MemberValue::Node(node) = MemberValue::from(&hybrid) else {
            panic!("expected a node");
        };
        let Hybrid::Node(hybrid_node) = &hybrid else {
            panic!("expected a node");
        };
        assert!(Arc::ptr_eq(&node.0, &hybrid_node.0));
        assert!(
            matches!(MemberValue::from(&Hybrid::from(0.5)), MemberValue::Float(value) if value == 0.5)
        );
    }
}
