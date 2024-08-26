use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper, Hybrid};

#[derive(Clone, Debug)]
pub struct FractalFBm<Source: Generator, Gain: Hybrid, WeightedStrength: Hybrid> {
    pub source: Source,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

#[derive(Clone, Debug)]
pub struct FractalRidged<Source: Generator, Gain: Hybrid, WeightedStrength: Hybrid> {
    pub source: Source,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

#[derive(Clone, Debug)]
pub struct FractalPingPong<
    Source: Generator,
    Gain: Hybrid,
    WeightedStrength: Hybrid,
    PingPongStrength: Hybrid,
> {
    pub source: Source,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub ping_pong_strength: PingPongStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

impl<Source: Generator, Gain: Hybrid, WeightedStrength: Hybrid> Generator
    for FractalFBm<Source, Gain, WeightedStrength>
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("FractalFBm").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator, Gain: Hybrid, WeightedStrength: Hybrid> Generator
    for FractalRidged<Source, Gain, WeightedStrength>
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("FractalRidged").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator, Gain: Hybrid, WeightedStrength: Hybrid, PingPongStrength: Hybrid> Generator
    for FractalPingPong<Source, Gain, WeightedStrength, PingPongStrength>
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("FractalFBm").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("PingPongStrength", self.ping_pong_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> GeneratorWrapper<Source> {
    pub fn fbm<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<FractalFBm<Source, Gain, WeightedStrength>> {
        FractalFBm {
            source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        }
        .into()
    }

    pub fn ridged<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<FractalRidged<Source, Gain, WeightedStrength>> {
        FractalRidged {
            source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        }
        .into()
    }

    pub fn ping_pong<Gain: Hybrid, WeightedStrength: Hybrid, PingPongStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        ping_pong_strength: PingPongStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<FractalPingPong<Source, Gain, WeightedStrength, PingPongStrength>> {
        FractalPingPong {
            source: self.0,
            gain,
            weighted_strength,
            ping_pong_strength,
            octaves,
            lacunarity,
        }
        .into()
    }
}
