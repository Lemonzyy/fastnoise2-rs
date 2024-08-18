use crate::{safe::SafeNode, Node};

use super::{Generator, Hybrid, TypedNode};

#[derive(Debug)]
pub struct FractalFBm<Source: TypedNode, Gain: Hybrid, WeightedStrength: Hybrid> {
    pub source: Source,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

#[derive(Debug)]
pub struct FractalRidged<Source: TypedNode, Gain: Hybrid, WeightedStrength: Hybrid> {
    pub source: Source,
    pub gain: Gain,
    pub weighted_strength: WeightedStrength,
    pub octaves: i32,
    pub lacunarity: f32,
}

#[derive(Debug)]
pub struct FractalPingPong<
    Source: TypedNode,
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

impl<Source: TypedNode, Gain: Hybrid, WeightedStrength: Hybrid> TypedNode
    for FractalFBm<Source, Gain, WeightedStrength>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("FractalFBm").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", &self.gain).unwrap();
        node.set("WeightedStrength", &self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node)
    }
}

impl<Source: TypedNode, Gain: Hybrid, WeightedStrength: Hybrid> TypedNode
    for FractalRidged<Source, Gain, WeightedStrength>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("FractalRidged").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", &self.gain).unwrap();
        node.set("WeightedStrength", &self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node)
    }
}

impl<Source: TypedNode, Gain: Hybrid, WeightedStrength: Hybrid, PingPongStrength: Hybrid> TypedNode
    for FractalPingPong<Source, Gain, WeightedStrength, PingPongStrength>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("FractalFBm").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", &self.gain).unwrap();
        node.set("WeightedStrength", &self.weighted_strength)
            .unwrap();
        node.set("PingPongStrength", &self.ping_pong_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node)
    }
}

impl<Source: TypedNode> Generator<Source> {
    pub fn fbm<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> Generator<FractalFBm<Source, Gain, WeightedStrength>> {
        Generator(FractalFBm {
            source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        })
    }

    pub fn ridged<Gain: Hybrid, WeightedStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> Generator<FractalRidged<Source, Gain, WeightedStrength>> {
        Generator(FractalRidged {
            source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        })
    }

    pub fn ping_pong<Gain: Hybrid, WeightedStrength: Hybrid, PingPongStrength: Hybrid>(
        self,
        gain: Gain,
        weighted_strength: WeightedStrength,
        ping_pong_strength: PingPongStrength,
        octaves: i32,
        lacunarity: f32,
    ) -> Generator<FractalPingPong<Source, Gain, WeightedStrength, PingPongStrength>> {
        Generator(FractalPingPong {
            source: self.0,
            gain,
            weighted_strength,
            ping_pong_strength,
            octaves,
            lacunarity,
        })
    }
}
