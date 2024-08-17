use crate::FastNoise;

use super::{Generator, Hybrid, Node};

#[derive(Clone, Copy, Debug)]
pub struct FractalFBm<Source: Node, Gain: Hybrid, WeightedStrength: Hybrid> {
    source: Source,
    gain: Gain,
    weighted_strength: WeightedStrength,
    octaves: i32,
    lacunarity: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct FractalRidged<Source: Node, Gain: Hybrid, WeightedStrength: Hybrid> {
    source: Source,
    gain: Gain,
    weighted_strength: WeightedStrength,
    octaves: i32,
    lacunarity: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct FractalPingPong<
    Source: Node,
    Gain: Hybrid,
    WeightedStrength: Hybrid,
    PingPongStrength: Hybrid,
> {
    source: Source,
    gain: Gain,
    weighted_strength: WeightedStrength,
    ping_pong_strength: PingPongStrength,
    octaves: i32,
    lacunarity: f32,
}

impl<Source: Node, Gain: Hybrid, WeightedStrength: Hybrid> Node
    for FractalFBm<Source, Gain, WeightedStrength>
{
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("FractalFBm").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Gain", self.gain).unwrap();
        node.set("WeightedStrength", self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        node
    }
}

impl<Source: Node, Gain: Hybrid, WeightedStrength: Hybrid> Node
    for FractalRidged<Source, Gain, WeightedStrength>
{
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("FractalRidged").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Gain", self.gain).unwrap();
        node.set("WeightedStrength", self.weighted_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        node
    }
}

impl<Source: Node, Gain: Hybrid, WeightedStrength: Hybrid, PingPongStrength: Hybrid> Node
    for FractalPingPong<Source, Gain, WeightedStrength, PingPongStrength>
{
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("FractalFBm").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Gain", self.gain).unwrap();
        node.set("WeightedStrength", self.weighted_strength)
            .unwrap();
        node.set("PingPongStrength", self.ping_pong_strength)
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        node
    }
}

pub fn fractal_fbm<Source: Node, Gain: Hybrid, WeightedStrength: Hybrid>(
    source: Source,
    gain: Gain,
    weighted_strength: WeightedStrength,
    octaves: i32,
    lacunarity: f32,
) -> Generator<FractalFBm<Source, Gain, WeightedStrength>> {
    Generator(FractalFBm {
        source,
        gain,
        weighted_strength,
        octaves,
        lacunarity,
    })
}

pub fn fractal_ridged<Source: Node, Gain: Hybrid, WeightedStrength: Hybrid>(
    source: Source,
    gain: Gain,
    weighted_strength: WeightedStrength,
    octaves: i32,
    lacunarity: f32,
) -> Generator<FractalRidged<Source, Gain, WeightedStrength>> {
    Generator(FractalRidged {
        source,
        gain,
        weighted_strength,
        octaves,
        lacunarity,
    })
}

pub fn fractal_ping_pong<
    Source: Node,
    Gain: Hybrid,
    WeightedStrength: Hybrid,
    PingPongStrength: Hybrid,
>(
    source: Source,
    gain: Gain,
    weighted_strength: WeightedStrength,
    ping_pong_strength: PingPongStrength,
    octaves: i32,
    lacunarity: f32,
) -> Generator<FractalPingPong<Source, Gain, WeightedStrength, PingPongStrength>> {
    Generator(FractalPingPong {
        source,
        gain,
        weighted_strength,
        ping_pong_strength,
        octaves,
        lacunarity,
    })
}
