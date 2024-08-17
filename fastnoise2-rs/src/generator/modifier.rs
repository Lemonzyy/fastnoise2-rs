use crate::FastNoise;

use super::{Dimension, Generator, Hybrid, Node};

#[derive(Clone, Copy, Debug)]
pub struct DomainScale<Source: Node> {
    source: Source,
    scale: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DomainOffset<
    Source: Node,
    XOffset: Hybrid,
    YOffset: Hybrid,
    ZOffset: Hybrid,
    WOffset: Hybrid,
> {
    source: Source,
    x_offset: XOffset,
    y_offset: YOffset,
    z_offset: ZOffset,
    w_offset: WOffset,
}

#[derive(Clone, Copy, Debug)]
pub struct DomainRotate<Source: Node> {
    source: Source,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct SeedOffset<Source: Node> {
    source: Source,
    seed_offset: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Remap<Source: Node> {
    source: Source,
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct ConvertRgba8<Source: Node> {
    source: Source,
    min: f32,
    max: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Terrace<Source: Node> {
    source: Source,
    multiplier: f32,
    smoothness: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DomainAxisScale<Source: Node> {
    source: Source,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    w_scale: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct AddDimension<Source: Node, NewDimensionPosition> {
    source: Source,
    new_dimension_position: NewDimensionPosition,
}

#[derive(Clone, Copy, Debug)]
pub struct RemoveDimension<Source: Node> {
    source: Source,
    remove_dimension: Dimension,
}

#[derive(Clone, Copy, Debug)]
pub struct GeneratorCache<Source: Node> {
    source: Source,
}

impl<Source: Node> Node for DomainScale<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("DomainScale").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Scale", self.scale).unwrap();
        node
    }
}

impl<Source: Node, XOffset: Hybrid, YOffset: Hybrid, ZOffset: Hybrid, WOffset: Hybrid> Node
    for DomainOffset<Source, XOffset, YOffset, ZOffset, WOffset>
{
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("DomainOffset").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("OffsetX", self.x_offset).unwrap();
        node.set("OffsetY", self.y_offset).unwrap();
        node.set("OffsetZ", self.z_offset).unwrap();
        node.set("OffsetW", self.w_offset).unwrap();
        node
    }
}

impl<Source: Node> Node for DomainRotate<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("DomainRotate").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Yaw", self.yaw).unwrap();
        node.set("Pitch", self.pitch).unwrap();
        node.set("Roll", self.roll).unwrap();
        node
    }
}

impl<Source: Node> Node for SeedOffset<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("SeedOffset").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        node
    }
}

impl<Source: Node> Node for Remap<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Remap").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("FromMin", self.from_min).unwrap();
        node.set("FromMax", self.from_max).unwrap();
        node.set("ToMin", self.to_min).unwrap();
        node.set("ToMax", self.to_max).unwrap();
        node
    }
}

impl<Source: Node> Node for ConvertRgba8<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("ConvertRgba8").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Min", self.min).unwrap();
        node.set("Max", self.max).unwrap();
        node
    }
}

impl<Source: Node> Node for Terrace<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Terrace").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Multiplier", self.multiplier).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        node
    }
}

impl<Source: Node> Node for DomainAxisScale<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("DomainAxisScale").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("ScaleX", self.x_scale).unwrap();
        node.set("ScaleY", self.y_scale).unwrap();
        node.set("ScaleZ", self.z_scale).unwrap();
        node.set("ScaleW", self.w_scale).unwrap();
        node
    }
}

impl<Source: Node, NewDimensionPosition: Hybrid> Node
    for AddDimension<Source, NewDimensionPosition>
{
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("AddDimension").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("NewDimensionPosition", self.new_dimension_position)
            .unwrap();
        node
    }
}

impl<Source: Node> Node for RemoveDimension<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("RemoveDimension").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("RemoveDimension", &*self.remove_dimension.to_string())
            .unwrap();
        node
    }
}

impl<Source: Node> Node for GeneratorCache<Source> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("GeneratorCache").unwrap();
        node.set("Source", self.source).unwrap();
        node
    }
}

pub fn domain_scale<Source: Node>(source: Source, scale: f32) -> Generator<DomainScale<Source>> {
    Generator(DomainScale { source, scale })
}

pub fn domain_offset<
    Source: Node,
    XOffset: Hybrid,
    YOffset: Hybrid,
    ZOffset: Hybrid,
    WOffset: Hybrid,
>(
    source: Source,
    x_offset: XOffset,
    y_offset: YOffset,
    z_offset: ZOffset,
    w_offset: WOffset,
) -> Generator<DomainOffset<Source, XOffset, YOffset, ZOffset, WOffset>> {
    Generator(DomainOffset {
        source,
        x_offset,
        y_offset,
        z_offset,
        w_offset,
    })
}

pub fn domain_rotate<Source: Node>(
    source: Source,
    yaw: f32,
    pitch: f32,
    roll: f32,
) -> Generator<DomainRotate<Source>> {
    Generator(DomainRotate {
        source,
        yaw,
        pitch,
        roll,
    })
}

pub fn seed_offset<Source: Node>(
    source: Source,
    seed_offset: i32,
) -> Generator<SeedOffset<Source>> {
    Generator(SeedOffset {
        source,
        seed_offset,
    })
}

pub fn remap<Source: Node>(
    source: Source,
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,
) -> Generator<Remap<Source>> {
    Generator(Remap {
        source,
        from_min,
        from_max,
        to_min,
        to_max,
    })
}

pub fn convert_rgba8<Source: Node>(
    source: Source,
    min: f32,
    max: f32,
) -> Generator<ConvertRgba8<Source>> {
    Generator(ConvertRgba8 { source, min, max })
}

pub fn terrace<Source: Node>(
    source: Source,
    multiplier: f32,
    smoothness: f32,
) -> Generator<Terrace<Source>> {
    Generator(Terrace {
        source,
        multiplier,
        smoothness,
    })
}

pub fn domain_axis_scale<Source: Node>(
    source: Source,
    scale: [f32; 4],
) -> Generator<DomainAxisScale<Source>> {
    let [x_scale, y_scale, z_scale, w_scale] = scale;
    Generator(DomainAxisScale {
        source,
        x_scale,
        y_scale,
        z_scale,
        w_scale,
    })
}

pub fn add_dimension<Source: Node, NewDimensionPosition: Hybrid>(
    source: Source,
    new_dimension_position: NewDimensionPosition,
) -> Generator<AddDimension<Source, NewDimensionPosition>> {
    Generator(AddDimension {
        source,
        new_dimension_position,
    })
}

pub fn remove_dimension<Source: Node>(
    source: Source,
    remove_dimension: Dimension,
) -> Generator<RemoveDimension<Source>> {
    Generator(RemoveDimension {
        source,
        remove_dimension,
    })
}

pub fn generator_cache<Source: Node>(source: Source) -> Generator<GeneratorCache<Source>> {
    Generator(GeneratorCache { source })
}
