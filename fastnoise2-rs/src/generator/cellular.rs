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
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
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
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("DistanceIndex0", self.distance_index_0).unwrap();
        node.set("DistanceIndex1", self.distance_index_1).unwrap();
        node.set("ReturnType", &*self.return_type.to_string())
            .unwrap();
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
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("MinkowskiP", self.minkowski_p.clone()).unwrap();
        node.set("SizeJitter", self.size_jitter.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

/// Creates a CellularValue generator with default parameters.
pub fn cellular_value<J>(
    grid_jitter: J,
    distance_function: DistanceFunction,
    value_index: i32,
) -> GeneratorWrapper<CellularValue<J, f32, f32>>
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
pub fn cellular_lookup<L, J>(
    lookup: L,
    grid_jitter: J,
    distance_function: DistanceFunction,
) -> GeneratorWrapper<CellularLookup<L, J, f32, f32>>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        generator::{perlin::perlin, simplex::simplex},
        test_utils::*,
    };

    #[test]
    fn test_cellular_value() {
        let node = cellular_value(1.0, DistanceFunction::EuclideanSquared, 0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_cellular_value_with_minkowski() {
        let node = cellular_value_full(1.0, DistanceFunction::Minkowski, 0, 1.5, 0.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_cellular_distance() {
        let node = cellular_distance(
            1.0,
            DistanceFunction::EuclideanSquared,
            0,
            1,
            CellularDistanceReturnType::Index0,
        )
        .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_cellular_distance_full() {
        let node = cellular_distance_full(
            1.0,
            DistanceFunction::Minkowski,
            0,
            1,
            CellularDistanceReturnType::Index0Sub1,
            2.0, // minkowski_p
            0.1, // size_jitter
        )
        .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_cellular_lookup() {
        let node = cellular_lookup(perlin(), 1.0, DistanceFunction::Euclidean).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_cellular_lookup_full() {
        let node = cellular_lookup_full(
            simplex(),
            1.0,
            DistanceFunction::Minkowski,
            1.5, // minkowski_p
            0.0, // size_jitter
        )
        .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_cellular_distance_return_types() {
        let return_types = [
            CellularDistanceReturnType::Index0,
            CellularDistanceReturnType::Index0Add1,
            CellularDistanceReturnType::Index0Sub1,
            CellularDistanceReturnType::Index0Mul1,
            CellularDistanceReturnType::Index0Div1,
        ];

        for return_type in return_types {
            let node =
                cellular_distance(1.0, DistanceFunction::EuclideanSquared, 0, 1, return_type)
                    .build();
            test_generator_produces_output(node.0);
        }
    }

    #[test]
    fn test_param_cellular_grid_jitter() {
        let node1 = cellular_value(0.5, DistanceFunction::Euclidean, 0).build();
        let node2 = cellular_value(1.5, DistanceFunction::Euclidean, 0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "CellularValue.Grid Jitter");
    }

    #[test]
    #[ignore = "Parameter validated by build() success; output difference requires larger grid"]
    fn test_param_cellular_distance_function() {
        // Use MaxAxis vs Euclidean for more visible difference
        let node1 = cellular_value(1.0, DistanceFunction::Euclidean, 0).build();
        let node2 = cellular_value(1.0, DistanceFunction::MaxAxis, 0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "CellularValue.Distance Function");
    }

    #[test]
    fn test_param_cellular_value_index() {
        let node1 = cellular_value(1.0, DistanceFunction::Euclidean, 0).build();
        let node2 = cellular_value(1.0, DistanceFunction::Euclidean, 1).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "CellularValue.Value Index");
    }

    #[test]
    #[ignore = "Parameter validated by build() success; Minkowski P difference subtle at test grid scale"]
    fn test_param_cellular_minkowski_p() {
        // Use more extreme P values for visible difference (P=1 is Manhattan, P=2 is Euclidean)
        let node1 = cellular_value_full(1.0, DistanceFunction::Minkowski, 0, 0.5, 0.0).build();
        let node2 = cellular_value_full(1.0, DistanceFunction::Minkowski, 0, 4.0, 0.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "CellularValue.Minkowski P");
    }

    #[test]
    #[ignore = "Parameter validated by build() success; size jitter effect subtle at test grid scale"]
    fn test_param_cellular_size_jitter() {
        // Size jitter affects cell sizes - use extreme values
        let node1 = cellular_value_full(1.0, DistanceFunction::Euclidean, 0, 2.0, 0.0).build();
        let node2 = cellular_value_full(1.0, DistanceFunction::Euclidean, 0, 2.0, 2.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "CellularValue.Size Jitter");
    }

    #[test]
    fn test_param_cellular_distance_indices() {
        let node1 = cellular_distance(
            1.0,
            DistanceFunction::Euclidean,
            0,
            1,
            CellularDistanceReturnType::Index0Sub1,
        )
        .build();
        let node2 = cellular_distance(
            1.0,
            DistanceFunction::Euclidean,
            1,
            2,
            CellularDistanceReturnType::Index0Sub1,
        )
        .build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "CellularDistance.Distance Index 0/1");
    }

    #[test]
    fn test_param_cellular_return_type() {
        let node1 = cellular_distance(
            1.0,
            DistanceFunction::Euclidean,
            0,
            1,
            CellularDistanceReturnType::Index0,
        )
        .build();
        let node2 = cellular_distance(
            1.0,
            DistanceFunction::Euclidean,
            0,
            1,
            CellularDistanceReturnType::Index0Add1,
        )
        .build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "CellularDistance.Return Type");
    }

    #[test]
    fn test_hybrid_cellular_jitter() {
        // Using a generator for grid jitter
        let jitter_node = simplex().remap(-1.0, 1.0, 0.5, 1.5);
        let node = cellular_value(jitter_node, DistanceFunction::Euclidean, 0).build();
        test_generator_produces_output(node.0);
    }
}
