use crate::{safe::SafeNode, Node};

use super::{domain_warp::DomainWarpNode, Generator, GeneratorWrapper, Hybrid};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

impl<DomainWarpSource: DomainWarpNode, Gain: Hybrid, WeightedStrength: Hybrid> Generator
    for DomainWarpFractalProgressive<DomainWarpSource, Gain, WeightedStrength>
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

impl<DomainWarpSource: DomainWarpNode, Gain: Hybrid, WeightedStrength: Hybrid> Generator
    for DomainWarpFractalIndependant<DomainWarpSource, Gain, WeightedStrength>
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

impl<DomainWarpSource: DomainWarpNode> GeneratorWrapper<DomainWarpSource> {
    pub fn warp_progressive<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<DomainWarpFractalProgressive<DomainWarpSource, Gain, WeightedStrength>>
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

    pub fn warp_independant<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<DomainWarpFractalIndependant<DomainWarpSource, Gain, WeightedStrength>>
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
