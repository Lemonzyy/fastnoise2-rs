use super::{DistanceFunction, Generator, GeneratorWrapper, Hybrid};
use crate::{safe::SafeNode, Node};

#[derive(Clone, Debug)]
pub struct CellularValue<J, M, S>
where
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    pub grid_jitter: J,
    pub distance_function: DistanceFunction,
    pub value_index: i32,
    pub minkowski_p: M,
    pub size_jitter: S,
}

#[derive(Clone, Debug)]
pub struct CellularDistance<J, M, S>
where
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    pub grid_jitter: J,
    pub distance_function: DistanceFunction,
    pub distance_index_0: i32,
    pub distance_index_1: i32,
    pub return_type: CellularDistanceReturnType,
    pub minkowski_p: M,
    pub size_jitter: S,
}

#[derive(Clone, Debug)]
pub struct CellularLookup<L, J, M, S>
where
    L: Generator,
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    pub lookup: L,
    pub grid_jitter: J,
    pub distance_function: DistanceFunction,
    pub minkowski_p: M,
    pub size_jitter: S,
}

impl<J, M, S> Generator for CellularValue<J, M, S>
where
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("CellularValue").unwrap();
        node.set("GridJitter", self.grid_jitter.clone()).unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string()).unwrap();
        node.set("ValueIndex", self.value_index).unwrap();
        node.set("MinkowskiP", self.minkowski_p.clone()).unwrap();
        node.set("SizeJitter", self.size_jitter.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<J, M, S> Generator for CellularDistance<J, M, S>
where
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("CellularDistance").unwrap();
        node.set("GridJitter", self.grid_jitter.clone()).unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string()).unwrap();
        node.set("DistanceIndex0", self.distance_index_0).unwrap();
        node.set("DistanceIndex1", self.distance_index_1).unwrap();
        node.set("ReturnType", &*self.return_type.to_string()).unwrap();
        node.set("MinkowskiP", self.minkowski_p.clone()).unwrap();
        node.set("SizeJitter", self.size_jitter.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<L, J, M, S> Generator for CellularLookup<L, J, M, S>
where
    L: Generator,
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("CellularLookup").unwrap();
        node.set("Lookup", &self.lookup).unwrap();
        node.set("GridJitter", self.grid_jitter.clone()).unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string()).unwrap();
        node.set("MinkowskiP", self.minkowski_p.clone()).unwrap();
        node.set("SizeJitter", self.size_jitter.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

/// Creates a CellularValue generator with default parameters.
pub fn cellular_value<J>(grid_jitter: J, distance_function: DistanceFunction, value_index: i32) -> GeneratorWrapper<CellularValue<J, f32, f32>>
where
    J: Hybrid,
{
    CellularValue {
        grid_jitter,
        distance_function,
        value_index,
        minkowski_p: 1.5,
        size_jitter: 0.0,
    }
    .into()
}

/// Creates a CellularValue generator with all parameters.
pub fn cellular_value_full<J, M, S>(
    grid_jitter: J,
    distance_function: DistanceFunction,
    value_index: i32,
    minkowski_p: M,
    size_jitter: S,
) -> GeneratorWrapper<CellularValue<J, M, S>>
where
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    CellularValue {
        grid_jitter,
        distance_function,
        value_index,
        minkowski_p,
        size_jitter,
    }
    .into()
}

/// Creates a CellularDistance generator with default parameters.
pub fn cellular_distance<J>(
    grid_jitter: J,
    distance_function: DistanceFunction,
    distance_index_0: i32,
    distance_index_1: i32,
    return_type: CellularDistanceReturnType,
) -> GeneratorWrapper<CellularDistance<J, f32, f32>>
where
    J: Hybrid,
{
    CellularDistance {
        grid_jitter,
        distance_function,
        distance_index_0,
        distance_index_1,
        return_type,
        minkowski_p: 1.5,
        size_jitter: 0.0,
    }
    .into()
}

/// Creates a CellularDistance generator with all parameters.
pub fn cellular_distance_full<J, M, S>(
    grid_jitter: J,
    distance_function: DistanceFunction,
    distance_index_0: i32,
    distance_index_1: i32,
    return_type: CellularDistanceReturnType,
    minkowski_p: M,
    size_jitter: S,
) -> GeneratorWrapper<CellularDistance<J, M, S>>
where
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    CellularDistance {
        grid_jitter,
        distance_function,
        distance_index_0,
        distance_index_1,
        return_type,
        minkowski_p,
        size_jitter,
    }
    .into()
}

/// Creates a CellularLookup generator with default parameters.
pub fn cellular_lookup<L, J>(lookup: L, grid_jitter: J, distance_function: DistanceFunction) -> GeneratorWrapper<CellularLookup<L, J, f32, f32>>
where
    L: Generator,
    J: Hybrid,
{
    CellularLookup {
        lookup,
        grid_jitter,
        distance_function,
        minkowski_p: 1.5,
        size_jitter: 0.0,
    }
    .into()
}

/// Creates a CellularLookup generator with all parameters.
pub fn cellular_lookup_full<L, J, M, S>(
    lookup: L,
    grid_jitter: J,
    distance_function: DistanceFunction,
    minkowski_p: M,
    size_jitter: S,
) -> GeneratorWrapper<CellularLookup<L, J, M, S>>
where
    L: Generator,
    J: Hybrid,
    M: Hybrid,
    S: Hybrid,
{
    CellularLookup {
        lookup,
        grid_jitter,
        distance_function,
        minkowski_p,
        size_jitter,
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
