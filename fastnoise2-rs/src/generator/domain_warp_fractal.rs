use crate::{safe::SafeNode, Node};

use super::{domain_warp::DomainWarpNode, Generator, GeneratorWrapper, Hybrid};

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
pub struct DomainWarpFractalIndependant<S, G, W>
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

impl<S, G, W> Generator for DomainWarpFractalIndependant<S, G, W>
where
    S: DomainWarpNode,
    G: Hybrid,
    W: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpFractalIndependant").unwrap();
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
    pub fn warp_progressive<G, W>(
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

    pub fn warp_independant<G, W>(
        self,
        gain: G,
        weighted_strength: W,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<DomainWarpFractalIndependant<S, G, W>>
    where
        G: Hybrid,
        W: Hybrid,
    {
        DomainWarpFractalIndependant {
            domain_warp_source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        }
        .into()
    }
}
