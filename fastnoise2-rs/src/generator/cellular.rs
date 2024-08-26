use crate::{safe::SafeNode, Node};

use super::{DistanceFunction, Generator, GeneratorWrapper, Hybrid};

#[derive(Clone, Debug)]
pub struct CellularValue<J>
where
    J: Hybrid,
{
    pub jitter_modifier: J,
    pub distance_function: DistanceFunction,
    pub value_index: i32,
}

#[derive(Clone, Debug)]
pub struct CellularDistance<J>
where
    J: Hybrid,
{
    pub jitter_modifier: J,
    pub distance_function: DistanceFunction,
    pub distance_index_0: i32,
    pub distance_index_1: i32,
    pub return_type: CellularDistanceReturnType,
}

#[derive(Clone, Debug)]
pub struct CellularLookup<L, J>
where
    L: Generator,
    J: Hybrid,
{
    pub lookup: L,
    pub jitter_modifier: J,
    pub distance_function: DistanceFunction,
    pub lookup_frequency: f32,
}

impl<J> Generator for CellularValue<J>
where
    J: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("CellularValue").unwrap();
        node.set("JitterModifier", self.jitter_modifier.clone())
            .unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("ValueIndex", self.value_index).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<J> Generator for CellularDistance<J>
where
    J: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("CellularValue").unwrap();
        node.set("JitterModifier", self.jitter_modifier.clone())
            .unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("DistanceIndex0", self.distance_index_0).unwrap();
        node.set("DistanceIndex1", self.distance_index_1).unwrap();
        node.set("ReturnType", &*self.return_type.to_string())
            .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<L, J> Generator for CellularLookup<L, J>
where
    L: Generator,
    J: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("CellularLookup").unwrap();
        node.set("Lookup", &self.lookup).unwrap();
        node.set("JitterModifier", self.jitter_modifier.clone())
            .unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("LookupFrequency", self.lookup_frequency).unwrap();
        SafeNode(node.into()).into()
    }
}

pub fn cellular_value<J>(
    jitter_modifier: J,
    distance_function: DistanceFunction,
    value_index: i32,
) -> GeneratorWrapper<CellularValue<J>>
where
    J: Hybrid,
{
    CellularValue {
        jitter_modifier,
        distance_function,
        value_index,
    }
    .into()
}

pub fn cellular_distance<J>(
    jitter_modifier: J,
    distance_function: DistanceFunction,
    distance_index_0: i32,
    distance_index_1: i32,
    return_type: CellularDistanceReturnType,
) -> GeneratorWrapper<CellularDistance<J>>
where
    J: Hybrid,
{
    CellularDistance {
        jitter_modifier,
        distance_function,
        distance_index_0,
        distance_index_1,
        return_type,
    }
    .into()
}

pub fn cellular_lookup<L, J>(
    lookup: L,
    jitter_modifier: J,
    distance_function: DistanceFunction,
    lookup_frequency: f32,
) -> GeneratorWrapper<CellularLookup<L, J>>
where
    L: Generator,
    J: Hybrid,
{
    CellularLookup {
        lookup,
        jitter_modifier,
        distance_function,
        lookup_frequency,
    }
    .into()
}

#[derive(Clone, Debug)]
pub enum CellularDistanceReturnType {
    Index0,
    Index0Add1,
    Index0Sub1,
    Index0Mul1,
    Index0Div1,
}

impl std::fmt::Display for CellularDistanceReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellularDistanceReturnType::Index0 => f.write_str("Index0"),
            CellularDistanceReturnType::Index0Add1 => f.write_str("Index0Add1"),
            CellularDistanceReturnType::Index0Sub1 => f.write_str("Index0Sub1"),
            CellularDistanceReturnType::Index0Mul1 => f.write_str("Index0Mul1"),
            CellularDistanceReturnType::Index0Div1 => f.write_str("Index0Div1"),
        }
    }
}
