use super::{DistanceFunction, Generator, GeneratorWrapper, Hybrid};
use crate::{safe::SafeNode, Node};

/// Constant value generator.
#[derive(Clone, Debug)]
pub struct Constant {
    pub value: f32,
}

/// White noise generator.
#[derive(Clone, Debug)]
pub struct White {
    /// Offset applied to the seed. Default: 0
    pub seed_offset: i32,
    /// Minimum output value. Default: -1.0
    pub output_min: f32,
    /// Maximum output value. Default: 1.0
    pub output_max: f32,
}

/// Checkerboard pattern generator.
#[derive(Clone, Debug)]
pub struct Checkerboard {
    /// Feature Scale (effectively 1/frequency). Default: 100.0
    pub feature_scale: f32,
    /// Minimum output value. Default: -1.0
    pub output_min: f32,
    /// Maximum output value. Default: 1.0
    pub output_max: f32,
}

/// Sine wave generator.
#[derive(Clone, Debug)]
pub struct SineWave {
    /// Feature Scale (effectively 1/frequency). Default: 100.0
    pub feature_scale: f32,
    /// Minimum output value. Default: -1.0
    pub output_min: f32,
    /// Maximum output value. Default: 1.0
    pub output_max: f32,
}

/// Gradient generator (formerly PositionOutput in older FastNoise2 versions).
/// Outputs a linear gradient based on position coordinates.
#[derive(Clone, Debug)]
pub struct Gradient {
    pub multiplier_x: f32,
    pub multiplier_y: f32,
    pub multiplier_z: f32,
    pub multiplier_w: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub offset_w: f32,
}

/// Distance to point generator.
/// Calculates distance from each point to a target point.
#[derive(Clone, Debug)]
pub struct DistanceToPoint<X, Y, Z, W, M>
where
    X: Hybrid,
    Y: Hybrid,
    Z: Hybrid,
    W: Hybrid,
    M: Hybrid,
{
    pub distance_function: DistanceFunction,
    /// X coordinate of target point (can be f32 or Generator).
    pub point_x: X,
    /// Y coordinate of target point (can be f32 or Generator).
    pub point_y: Y,
    /// Z coordinate of target point (can be f32 or Generator).
    pub point_z: Z,
    /// W coordinate of target point (can be f32 or Generator).
    pub point_w: W,
    /// Minkowski P value for Minkowski distance function. Default: 1.5
    pub minkowski_p: M,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            multiplier_x: 0.0,
            multiplier_y: 0.0,
            multiplier_z: 0.0,
            multiplier_w: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
            offset_w: 0.0,
        }
    }
}

impl Default for White {
    fn default() -> Self {
        Self {
            seed_offset: 0,
            output_min: -1.0,
            output_max: 1.0,
        }
    }
}

impl Default for DistanceToPoint<f32, f32, f32, f32, f32> {
    fn default() -> Self {
        Self {
            distance_function: DistanceFunction::EuclideanSquared,
            point_x: 0.0,
            point_y: 0.0,
            point_z: 0.0,
            point_w: 0.0,
            minkowski_p: 1.5,
        }
    }
}

impl Default for Checkerboard {
    fn default() -> Self {
        Self {
            feature_scale: 100.0,
            output_min: -1.0,
            output_max: 1.0,
        }
    }
}

impl Default for SineWave {
    fn default() -> Self {
        Self {
            feature_scale: 100.0,
            output_min: -1.0,
            output_max: 1.0,
        }
    }
}

impl Generator for Constant {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Constant").unwrap();
        node.set("Value", self.value).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for White {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("White").unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        node.set("OutputMin", self.output_min).unwrap();
        node.set("OutputMax", self.output_max).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for Checkerboard {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Checkerboard").unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set("OutputMin", self.output_min).unwrap();
        node.set("OutputMax", self.output_max).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for SineWave {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("SineWave").unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set("OutputMin", self.output_min).unwrap();
        node.set("OutputMax", self.output_max).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for Gradient {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Gradient").unwrap();
        node.set("MultiplierX", self.multiplier_x).unwrap();
        node.set("MultiplierY", self.multiplier_y).unwrap();
        node.set("MultiplierZ", self.multiplier_z).unwrap();
        node.set("MultiplierW", self.multiplier_w).unwrap();
        node.set("OffsetX", self.offset_x).unwrap();
        node.set("OffsetY", self.offset_y).unwrap();
        node.set("OffsetZ", self.offset_z).unwrap();
        node.set("OffsetW", self.offset_w).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<X, Y, Z, W, M> Generator for DistanceToPoint<X, Y, Z, W, M>
where
    X: Hybrid,
    Y: Hybrid,
    Z: Hybrid,
    W: Hybrid,
    M: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DistanceToPoint").unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("PointX", self.point_x.clone()).unwrap();
        node.set("PointY", self.point_y.clone()).unwrap();
        node.set("PointZ", self.point_z.clone()).unwrap();
        node.set("PointW", self.point_w.clone()).unwrap();
        node.set("MinkowskiP", self.minkowski_p.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

/// Creates a constant value generator.
pub fn constant(value: f32) -> GeneratorWrapper<Constant> {
    Constant { value }.into()
}

/// Creates a white noise generator with default parameters.
pub fn white() -> GeneratorWrapper<White> {
    White::default().into()
}

/// Creates a checkerboard pattern generator with the given feature scale.
pub fn checkerboard(feature_scale: f32) -> GeneratorWrapper<Checkerboard> {
    Checkerboard {
        feature_scale,
        ..Default::default()
    }
    .into()
}

/// Creates a sine wave generator with the given feature scale.
pub fn sinewave(feature_scale: f32) -> GeneratorWrapper<SineWave> {
    SineWave {
        feature_scale,
        ..Default::default()
    }
    .into()
}

/// Creates a Gradient generator with default parameters (all multipliers = 0.0, all offsets = 0.0).
pub fn gradient() -> GeneratorWrapper<Gradient> {
    Gradient::default().into()
}

/// Creates a distance to point generator with all default parameters.
pub fn distance_to_point() -> GeneratorWrapper<DistanceToPoint<f32, f32, f32, f32, f32>> {
    DistanceToPoint::default().into()
}

// Builder methods for White
impl GeneratorWrapper<White> {
    /// Sets the seed offset for variation.
    pub fn with_seed_offset(mut self, offset: i32) -> Self {
        self.0.seed_offset = offset;
        self
    }

    /// Sets the output range.
    pub fn with_output_range(mut self, min: f32, max: f32) -> Self {
        self.0.output_min = min;
        self.0.output_max = max;
        self
    }
}

// Builder methods for Checkerboard
impl GeneratorWrapper<Checkerboard> {
    /// Sets the feature scale (effectively 1/frequency).
    pub fn with_feature_scale(mut self, scale: f32) -> Self {
        self.0.feature_scale = scale;
        self
    }

    /// Sets the output range.
    pub fn with_output_range(mut self, min: f32, max: f32) -> Self {
        self.0.output_min = min;
        self.0.output_max = max;
        self
    }
}

// Builder methods for SineWave
impl GeneratorWrapper<SineWave> {
    /// Sets the feature scale (effectively 1/frequency).
    pub fn with_feature_scale(mut self, scale: f32) -> Self {
        self.0.feature_scale = scale;
        self
    }

    /// Sets the output range.
    pub fn with_output_range(mut self, min: f32, max: f32) -> Self {
        self.0.output_min = min;
        self.0.output_max = max;
        self
    }
}

// Builder methods for Gradient
impl GeneratorWrapper<Gradient> {
    /// Sets the X multiplier for the gradient.
    pub fn with_multiplier_x(mut self, multiplier: f32) -> Self {
        self.0.multiplier_x = multiplier;
        self
    }

    /// Sets the Y multiplier for the gradient.
    pub fn with_multiplier_y(mut self, multiplier: f32) -> Self {
        self.0.multiplier_y = multiplier;
        self
    }

    /// Sets the Z multiplier for the gradient.
    pub fn with_multiplier_z(mut self, multiplier: f32) -> Self {
        self.0.multiplier_z = multiplier;
        self
    }

    /// Sets the W multiplier for the gradient.
    pub fn with_multiplier_w(mut self, multiplier: f32) -> Self {
        self.0.multiplier_w = multiplier;
        self
    }

    /// Sets all multipliers at once.
    pub fn with_multipliers(mut self, multipliers: [f32; 4]) -> Self {
        let [mx, my, mz, mw] = multipliers;
        self.0.multiplier_x = mx;
        self.0.multiplier_y = my;
        self.0.multiplier_z = mz;
        self.0.multiplier_w = mw;
        self
    }

    /// Sets the X offset for the gradient.
    pub fn with_offset_x(mut self, offset: f32) -> Self {
        self.0.offset_x = offset;
        self
    }

    /// Sets the Y offset for the gradient.
    pub fn with_offset_y(mut self, offset: f32) -> Self {
        self.0.offset_y = offset;
        self
    }

    /// Sets the Z offset for the gradient.
    pub fn with_offset_z(mut self, offset: f32) -> Self {
        self.0.offset_z = offset;
        self
    }

    /// Sets the W offset for the gradient.
    pub fn with_offset_w(mut self, offset: f32) -> Self {
        self.0.offset_w = offset;
        self
    }

    /// Sets all offsets at once.
    pub fn with_offsets(mut self, offsets: [f32; 4]) -> Self {
        let [ox, oy, oz, ow] = offsets;
        self.0.offset_x = ox;
        self.0.offset_y = oy;
        self.0.offset_z = oz;
        self.0.offset_w = ow;
        self
    }
}

// Builder methods for DistanceToPoint with f32 coordinates
impl GeneratorWrapper<DistanceToPoint<f32, f32, f32, f32, f32>> {
    /// Sets the distance function for distance calculation.
    pub fn with_distance_function(mut self, distance_function: DistanceFunction) -> Self {
        self.0.distance_function = distance_function;
        self
    }

    /// Sets the minkowski P value for Minkowski distance function.
    pub fn with_minkowski_p<M: Hybrid>(
        self,
        minkowski_p: M,
    ) -> GeneratorWrapper<DistanceToPoint<f32, f32, f32, f32, M>> {
        DistanceToPoint {
            distance_function: self.0.distance_function,
            point_x: self.0.point_x,
            point_y: self.0.point_y,
            point_z: self.0.point_z,
            point_w: self.0.point_w,
            minkowski_p,
        }
        .into()
    }

    /// Sets the X coordinate of the target point (can be f32 or Generator).
    pub fn with_point_x<X: Hybrid>(
        self,
        point_x: X,
    ) -> GeneratorWrapper<DistanceToPoint<X, f32, f32, f32, f32>> {
        DistanceToPoint {
            distance_function: self.0.distance_function,
            point_x,
            point_y: self.0.point_y,
            point_z: self.0.point_z,
            point_w: self.0.point_w,
            minkowski_p: self.0.minkowski_p,
        }
        .into()
    }

    /// Sets the Y coordinate of the target point (can be f32 or Generator).
    pub fn with_point_y<Y: Hybrid>(
        self,
        point_y: Y,
    ) -> GeneratorWrapper<DistanceToPoint<f32, Y, f32, f32, f32>> {
        DistanceToPoint {
            distance_function: self.0.distance_function,
            point_x: self.0.point_x,
            point_y,
            point_z: self.0.point_z,
            point_w: self.0.point_w,
            minkowski_p: self.0.minkowski_p,
        }
        .into()
    }

    /// Sets the Z coordinate of the target point (can be f32 or Generator).
    pub fn with_point_z<Z: Hybrid>(
        self,
        point_z: Z,
    ) -> GeneratorWrapper<DistanceToPoint<f32, f32, Z, f32, f32>> {
        DistanceToPoint {
            distance_function: self.0.distance_function,
            point_x: self.0.point_x,
            point_y: self.0.point_y,
            point_z,
            point_w: self.0.point_w,
            minkowski_p: self.0.minkowski_p,
        }
        .into()
    }

    /// Sets the W coordinate of the target point (can be f32 or Generator).
    pub fn with_point_w<W: Hybrid>(
        self,
        point_w: W,
    ) -> GeneratorWrapper<DistanceToPoint<f32, f32, f32, W, f32>> {
        DistanceToPoint {
            distance_function: self.0.distance_function,
            point_x: self.0.point_x,
            point_y: self.0.point_y,
            point_z: self.0.point_z,
            point_w,
            minkowski_p: self.0.minkowski_p,
        }
        .into()
    }

    /// Sets all point coordinates at once.
    pub fn with_point(mut self, point: [f32; 4]) -> Self {
        let [px, py, pz, pw] = point;
        self.0.point_x = px;
        self.0.point_y = py;
        self.0.point_z = pz;
        self.0.point_w = pw;
        self
    }
}
