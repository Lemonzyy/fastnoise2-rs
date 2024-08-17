use crate::FastNoise;

use super::{DistanceFunction, Generator, Hybrid, Node};

#[derive(Clone, Copy, Debug)]
pub struct CellularValue<JitterModifier: Hybrid> {
    jitter_modifier: JitterModifier,
    distance_function: DistanceFunction,
    value_index: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct CellularDistance<JitterModifier: Hybrid> {
    jitter_modifier: JitterModifier,
    distance_function: DistanceFunction,
    distance_index_0: i32,
    distance_index_1: i32,
    return_type: CellularDistanceReturnType,
}

#[derive(Clone, Copy, Debug)]
pub struct CellularLookup<Lookup: Node, JitterModifier: Hybrid> {
    lookup: Lookup,
    jitter_modifier: JitterModifier,
    distance_function: DistanceFunction,
    lookup_frequency: f32,
}

impl<JitterModifier: Hybrid> Node for CellularValue<JitterModifier> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("CellularValue").unwrap();
        node.set("JitterModifier", self.jitter_modifier).unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("ValueIndex", self.value_index).unwrap();
        node
    }
}

impl<JitterModifier: Hybrid> Node for CellularDistance<JitterModifier> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("CellularValue").unwrap();
        node.set("JitterModifier", self.jitter_modifier).unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("DistanceIndex0", self.distance_index_0).unwrap();
        node.set("DistanceIndex1", self.distance_index_1).unwrap();
        node.set("ReturnType", &*self.return_type.to_string())
            .unwrap();
        node
    }
}

impl<Lookup: Node, JitterModifier: Hybrid> Node for CellularLookup<Lookup, JitterModifier> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("CellularLookup").unwrap();
        node.set("Lookup", self.lookup).unwrap();
        node.set("JitterModifier", self.jitter_modifier).unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("LookupFrequency", self.lookup_frequency).unwrap();
        node
    }
}

pub fn cellular_value<JitterModifier: Hybrid>(
    jitter_modifier: JitterModifier,
    distance_function: DistanceFunction,
    value_index: i32,
) -> Generator<CellularValue<JitterModifier>> {
    Generator(CellularValue {
        jitter_modifier,
        distance_function,
        value_index,
    })
}

pub fn cellular_distance<JitterModifier: Hybrid>(
    jitter_modifier: JitterModifier,
    distance_function: DistanceFunction,
    distance_index_0: i32,
    distance_index_1: i32,
    return_type: CellularDistanceReturnType,
) -> Generator<CellularDistance<JitterModifier>> {
    Generator(CellularDistance {
        jitter_modifier,
        distance_function,
        distance_index_0,
        distance_index_1,
        return_type,
    })
}

pub fn cellular_lookup<Lookup: Node, JitterModifier: Hybrid>(
    lookup: Lookup,
    jitter_modifier: JitterModifier,
    distance_function: DistanceFunction,
    lookup_frequency: f32,
) -> Generator<CellularLookup<Lookup, JitterModifier>> {
    Generator(CellularLookup {
        lookup,
        jitter_modifier,
        distance_function,
        lookup_frequency,
    })
}

#[derive(Clone, Copy, Debug)]
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
