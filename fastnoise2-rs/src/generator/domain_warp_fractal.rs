use super::{domain_warp::DomainWarpNode, Generator, GeneratorWrapper, Hybrid};
use crate::{safe::SafeNode, Node};

#[derive(Clone, Debug)]
pub struct DomainWarpFractalProgressive<S, G, W>
where
    S: DomainWarpNode,
    G: Hybrid,
    W: Hybrid,
{
    pub domain_warp_source: S,
    pub gain: G,
    pub weighted_strength: W,
    pub octaves: i32,
    pub lacunarity: f32,
}

#[derive(Clone, Debug)]
pub struct DomainWarpFractalIndependent<S, G, W>
where
    S: DomainWarpNode,
    G: Hybrid,
    W: Hybrid,
{
    pub domain_warp_source: S,
    pub gain: G,
    pub weighted_strength: W,
    pub octaves: i32,
    pub lacunarity: f32,
}

impl<S, G, W> Generator for DomainWarpFractalProgressive<S, G, W>
where
    S: DomainWarpNode,
    G: Hybrid,
    W: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpFractalProgressive").unwrap();
        node.set("DomainWarpSource", &self.domain_warp_source)
            .unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, G, W> Generator for DomainWarpFractalIndependent<S, G, W>
where
    S: DomainWarpNode,
    G: Hybrid,
    W: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpFractalIndependent").unwrap();
        node.set("DomainWarpSource", &self.domain_warp_source)
            .unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> GeneratorWrapper<S>
where
    S: DomainWarpNode,
{
    pub fn domain_warp_progressive<G, W>(
        self,
        gain: G,
        weighted_strength: W,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<DomainWarpFractalProgressive<S, G, W>>
    where
        G: Hybrid,
        W: Hybrid,
    {
        DomainWarpFractalProgressive {
            domain_warp_source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        }
        .into()
    }

    pub fn domain_warp_independent<G, W>(
        self,
        gain: G,
        weighted_strength: W,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<DomainWarpFractalIndependent<S, G, W>>
    where
        G: Hybrid,
        W: Hybrid,
    {
        DomainWarpFractalIndependent {
            domain_warp_source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generator::perlin::perlin, test_utils::*};

    #[test]
    fn test_domain_warp_fractal_progressive() {
        // Domain warp fractal methods work on DomainWarpNode types
        let node = perlin()
            .domain_warp_gradient(50.0, 100.0)
            .domain_warp_progressive(0.5, 0.0, 4, 2.0)
            .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_domain_warp_fractal_independent() {
        let node = perlin()
            .domain_warp_gradient(50.0, 100.0)
            .domain_warp_independent(0.5, 0.0, 4, 2.0)
            .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_param_domain_warp_fractal_gain() {
        let node1 = perlin()
            .domain_warp_gradient(50.0, 100.0)
            .domain_warp_progressive(0.3, 0.0, 4, 2.0)
            .build();
        let node2 = perlin()
            .domain_warp_gradient(50.0, 100.0)
            .domain_warp_progressive(0.7, 0.0, 4, 2.0)
            .build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "DomainWarpFractalProgressive.Gain");
    }

    #[test]
    fn test_param_domain_warp_fractal_weighted_strength() {
        let node1 = perlin()
            .domain_warp_gradient(50.0, 100.0)
            .domain_warp_progressive(0.5, 0.0, 4, 2.0)
            .build();
        let node2 = perlin()
            .domain_warp_gradient(50.0, 100.0)
            .domain_warp_progressive(0.5, 0.5, 4, 2.0)
            .build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(
            &output1,
            &output2,
            "DomainWarpFractalProgressive.Weighted Strength",
        );
    }
}
