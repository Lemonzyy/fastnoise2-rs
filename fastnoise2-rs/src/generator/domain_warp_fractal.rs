use crate::{safe::SafeNode, Node};

use super::{domain_warp::DomainWarpNode, Generator, Hybrid, TypedNode};

#[derive(Clone, Copy, Debug)]
pub struct DomainWarpFractalProgressive<
    DomainWarpSource: DomainWarpNode,
    Gain: Hybrid,
    WeightedStrength: Hybrid,
> {
    pub domain_warp_source: DomainWarpSource,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DomainWarpFractalIndependant<
    DomainWarpSource: DomainWarpNode,
    Gain: Hybrid,
    WeightedStrength: Hybrid,
> {
    pub domain_warp_source: DomainWarpSource,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

impl<DomainWarpSource: DomainWarpNode, Gain: Hybrid, WeightedStrength: Hybrid> TypedNode
    for DomainWarpFractalProgressive<DomainWarpSource, Gain, WeightedStrength>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("DomainWarpFractalProgressive").unwrap();
        node.set("DomainWarpSource", self.domain_warp_source)
            .unwrap();
        node.set("Gain", self.gain).unwrap();
        node.set("WeightedStrength", self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node)
    }
}

impl<DomainWarpSource: DomainWarpNode, Gain: Hybrid, WeightedStrength: Hybrid> TypedNode
    for DomainWarpFractalIndependant<DomainWarpSource, Gain, WeightedStrength>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("DomainWarpFractalIndependant").unwrap();
        node.set("DomainWarpSource", self.domain_warp_source)
            .unwrap();
        node.set("Gain", self.gain).unwrap();
        node.set("WeightedStrength", self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node)
    }
}

impl<DomainWarpSource: DomainWarpNode> Generator<DomainWarpSource> {
    pub fn warp_progressive<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> Generator<DomainWarpFractalProgressive<DomainWarpSource, Gain, WeightedStrength>> {
        Generator(DomainWarpFractalProgressive {
            domain_warp_source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        })
    }

    pub fn warp_independant<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> Generator<DomainWarpFractalIndependant<DomainWarpSource, Gain, WeightedStrength>> {
        Generator(DomainWarpFractalIndependant {
            domain_warp_source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        })
    }
}
