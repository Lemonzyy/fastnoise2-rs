use crate::{typed::TypedFastNoise, FastNoise};

use super::{Generator, Hybrid, Node};

#[derive(Clone, Copy, Debug)]
pub struct DomainWarpFractalProgressive<
    DomainWarpSource: Node,
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
    DomainWarpSource: Node,
    Gain: Hybrid,
    WeightedStrength: Hybrid,
> {
    pub domain_warp_source: DomainWarpSource,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

impl<DomainWarpSource: Node, Gain: Hybrid, WeightedStrength: Hybrid> Node
    for DomainWarpFractalProgressive<DomainWarpSource, Gain, WeightedStrength>
{
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("DomainWarpFractalProgressive").unwrap();
        node.set("DomainWarpSource", self.domain_warp_source)
            .unwrap();
        node.set("Gain", self.gain).unwrap();
        node.set("WeightedStrength", self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        TypedFastNoise(node)
    }
}

impl<DomainWarpSource: Node, Gain: Hybrid, WeightedStrength: Hybrid> Node
    for DomainWarpFractalIndependant<DomainWarpSource, Gain, WeightedStrength>
{
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("DomainWarpFractalIndependant").unwrap();
        node.set("DomainWarpSource", self.domain_warp_source)
            .unwrap();
        node.set("Gain", self.gain).unwrap();
        node.set("WeightedStrength", self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        TypedFastNoise(node)
    }
}

pub fn progressive<DomainWarpSource: Node, Gain: Hybrid, WeightedStrength: Hybrid>(
    domain_warp_source: DomainWarpSource,
    gain: Gain,
    weighted_strength: WeightedStrength,
    octaves: i32,
    lacunarity: f32,
) -> Generator<DomainWarpFractalProgressive<DomainWarpSource, Gain, WeightedStrength>> {
    Generator(DomainWarpFractalProgressive {
        domain_warp_source,
        gain,
        weighted_strength,
        octaves,
        lacunarity,
    })
}

pub fn independant<DomainWarpSource: Node, Gain: Hybrid, WeightedStrength: Hybrid>(
    domain_warp_source: DomainWarpSource,
    gain: Gain,
    weighted_strength: WeightedStrength,
    octaves: i32,
    lacunarity: f32,
) -> Generator<DomainWarpFractalIndependant<DomainWarpSource, Gain, WeightedStrength>> {
    Generator(DomainWarpFractalIndependant {
        domain_warp_source,
        gain,
        weighted_strength,
        octaves,
        lacunarity,
    })
}
