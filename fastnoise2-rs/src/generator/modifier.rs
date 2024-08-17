use crate::{typed::TypedFastNoise, FastNoise};

use super::{Dimension, Generator, Hybrid, Node};

#[derive(Clone, Copy, Debug)]
pub struct DomainScale<Source: Node> {
    pub source: Source,
    pub scale: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DomainOffset<
    Source: Node,
    XOffset: Hybrid,
    YOffset: Hybrid,
    ZOffset: Hybrid,
    WOffset: Hybrid,
> {
    pub source: Source,
    pub x_offset: XOffset,
    pub y_offset: YOffset,
    pub z_offset: ZOffset,
    pub w_offset: WOffset,
}

#[derive(Clone, Copy, Debug)]
pub struct DomainRotate<Source: Node> {
    pub source: Source,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct SeedOffset<Source: Node> {
    pub source: Source,
    pub seed_offset: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Remap<Source: Node> {
    pub source: Source,
    pub from_min: f32,
    pub from_max: f32,
    pub to_min: f32,
    pub to_max: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct ConvertRgba8<Source: Node> {
    pub source: Source,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Terrace<Source: Node> {
    pub source: Source,
    pub multiplier: f32,
    pub smoothness: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DomainAxisScale<Source: Node> {
    pub source: Source,
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub w_scale: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct AddDimension<Source: Node, NewDimensionPosition> {
    pub source: Source,
    pub new_dimension_position: NewDimensionPosition,
}

#[derive(Clone, Copy, Debug)]
pub struct RemoveDimension<Source: Node> {
    pub source: Source,
    pub remove_dimension: Dimension,
}

#[derive(Clone, Copy, Debug)]
pub struct GeneratorCache<Source: Node> {
    pub source: Source,
}

impl<Source: Node> Node for DomainScale<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("DomainScale").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Scale", self.scale).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node, XOffset: Hybrid, YOffset: Hybrid, ZOffset: Hybrid, WOffset: Hybrid> Node
    for DomainOffset<Source, XOffset, YOffset, ZOffset, WOffset>
{
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("DomainOffset").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("OffsetX", self.x_offset).unwrap();
        node.set("OffsetY", self.y_offset).unwrap();
        node.set("OffsetZ", self.z_offset).unwrap();
        node.set("OffsetW", self.w_offset).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for DomainRotate<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("DomainRotate").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Yaw", self.yaw).unwrap();
        node.set("Pitch", self.pitch).unwrap();
        node.set("Roll", self.roll).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for SeedOffset<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("SeedOffset").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for Remap<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("Remap").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("FromMin", self.from_min).unwrap();
        node.set("FromMax", self.from_max).unwrap();
        node.set("ToMin", self.to_min).unwrap();
        node.set("ToMax", self.to_max).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for ConvertRgba8<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("ConvertRgba8").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Min", self.min).unwrap();
        node.set("Max", self.max).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for Terrace<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("Terrace").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("Multiplier", self.multiplier).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for DomainAxisScale<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("DomainAxisScale").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("ScaleX", self.x_scale).unwrap();
        node.set("ScaleY", self.y_scale).unwrap();
        node.set("ScaleZ", self.z_scale).unwrap();
        node.set("ScaleW", self.w_scale).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node, NewDimensionPosition: Hybrid> Node
    for AddDimension<Source, NewDimensionPosition>
{
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("AddDimension").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("NewDimensionPosition", self.new_dimension_position)
            .unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for RemoveDimension<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("RemoveDimension").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("RemoveDimension", &*self.remove_dimension.to_string())
            .unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Node for GeneratorCache<Source> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("GeneratorCache").unwrap();
        node.set("Source", self.source).unwrap();
        TypedFastNoise(node)
    }
}

impl<Source: Node> Generator<Source> {
    pub fn scale(self, scale: f32) -> Generator<DomainScale<Source>> {
        Generator(DomainScale {
            source: self.0,
            scale,
        })
    }

    pub fn offset<XOffset: Hybrid, YOffset: Hybrid, ZOffset: Hybrid, WOffset: Hybrid>(
        self,
        x_offset: XOffset,
        y_offset: YOffset,
        z_offset: ZOffset,
        w_offset: WOffset,
    ) -> Generator<DomainOffset<Source, XOffset, YOffset, ZOffset, WOffset>> {
        Generator(DomainOffset {
            source: self.0,
            x_offset,
            y_offset,
            z_offset,
            w_offset,
        })
    }

    pub fn rotate(self, yaw: f32, pitch: f32, roll: f32) -> Generator<DomainRotate<Source>> {
        Generator(DomainRotate {
            source: self.0,
            yaw,
            pitch,
            roll,
        })
    }

    pub fn seed_offset(self, seed_offset: i32) -> Generator<SeedOffset<Source>> {
        Generator(SeedOffset {
            source: self.0,
            seed_offset,
        })
    }

    pub fn remap(
        self,
        from_min: f32,
        from_max: f32,
        to_min: f32,
        to_max: f32,
    ) -> Generator<Remap<Source>> {
        Generator(Remap {
            source: self.0,
            from_min,
            from_max,
            to_min,
            to_max,
        })
    }

    pub fn convert_rgba8(self, min: f32, max: f32) -> Generator<ConvertRgba8<Source>> {
        Generator(ConvertRgba8 {
            source: self.0,
            min,
            max,
        })
    }

    pub fn terrace(self, multiplier: f32, smoothness: f32) -> Generator<Terrace<Source>> {
        Generator(Terrace {
            source: self.0,
            multiplier,
            smoothness,
        })
    }

    pub fn axis_scale(self, scale: [f32; 4]) -> Generator<DomainAxisScale<Source>> {
        let [x_scale, y_scale, z_scale, w_scale] = scale;
        Generator(DomainAxisScale {
            source: self.0,
            x_scale,
            y_scale,
            z_scale,
            w_scale,
        })
    }

    pub fn add_dimension<NewDimensionPosition: Hybrid>(
        self,
        new_dimension_position: NewDimensionPosition,
    ) -> Generator<AddDimension<Source, NewDimensionPosition>> {
        Generator(AddDimension {
            source: self.0,
            new_dimension_position,
        })
    }

    pub fn remove_dimension(
        self,
        remove_dimension: Dimension,
    ) -> Generator<RemoveDimension<Source>> {
        Generator(RemoveDimension {
            source: self.0,
            remove_dimension,
        })
    }

    pub fn cache(self) -> Generator<GeneratorCache<Source>> {
        Generator(GeneratorCache { source: self.0 })
    }
}
