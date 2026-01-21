use super::{FadeInterpolation, Generator, GeneratorWrapper, Hybrid};
use crate::{safe::SafeNode, Node};

#[derive(Clone, Debug)]
pub struct Add<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Subtract<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Multiply<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Divide<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Modulus<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Min<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Max<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct MinSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
    pub smoothness: S,
}

#[derive(Clone, Debug)]
pub struct MaxSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
    pub smoothness: S,
}

#[derive(Clone, Debug)]
pub struct Fade<A, B, F, FMin, FMax>
where
    A: Generator,
    B: Generator,
    F: Hybrid,
    FMin: Hybrid,
    FMax: Hybrid,
{
    pub a: A,
    pub b: B,
    pub fade: F,
    pub fade_min: FMin,
    pub fade_max: FMax,
    pub interpolation: FadeInterpolation,
}

#[derive(Clone, Debug)]
pub struct PowFloat<V, P>
where
    V: Hybrid,
    P: Hybrid,
{
    pub value: V,
    pub pow: P,
}

#[derive(Clone, Debug)]
pub struct PowInt<V>
where
    V: Generator,
{
    pub value: V,
    pub pow: i32,
}

impl<Lhs, Rhs> Generator for Add<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Add").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Subtract<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Subtract").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Multiply<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Multiply").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Divide<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Divide").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Modulus<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Modulus").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Min<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Min").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Max<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Max").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs, S> Generator for MinSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("MinSmooth").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        node.set("Smoothness", self.smoothness.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs, S> Generator for MaxSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("MaxSmooth").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        node.set("Smoothness", self.smoothness.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<A, B, F, FMin, FMax> Generator for Fade<A, B, F, FMin, FMax>
where
    A: Generator,
    B: Generator,
    F: Hybrid,
    FMin: Hybrid,
    FMax: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Fade").unwrap();
        node.set("A", &self.a).unwrap();
        node.set("B", &self.b).unwrap();
        node.set("Fade", self.fade.clone()).unwrap();
        node.set("FadeMin", self.fade_min.clone()).unwrap();
        node.set("FadeMax", self.fade_max.clone()).unwrap();
        node.set("Interpolation", &*self.interpolation.to_string())
            .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<V, P> Generator for PowFloat<V, P>
where
    V: Hybrid,
    P: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("PowFloat").unwrap();
        node.set("Value", self.value.clone()).unwrap();
        node.set("Pow", self.pow.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<V> Generator for PowInt<V>
where
    V: Generator,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("PowInt").unwrap();
        node.set("Value", &self.value).unwrap();
        node.set("Pow", self.pow).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> std::ops::Add<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Add<Lhs, Rhs>>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Add { lhs: self.0, rhs }.into()
    }
}

impl<Lhs, Rhs> std::ops::Sub<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Subtract<Lhs, Rhs>>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        Subtract { lhs: self.0, rhs }.into()
    }
}

impl<Lhs, Rhs> std::ops::Mul<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Multiply<Lhs, Rhs>>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Multiply { lhs: self.0, rhs }.into()
    }
}

impl<Lhs> std::ops::Neg for GeneratorWrapper<Lhs>
where
    Lhs: Generator,
{
    type Output = GeneratorWrapper<Multiply<Lhs, f32>>;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<Lhs, Rhs> std::ops::Div<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Divide<Lhs, Rhs>>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Divide { lhs: self.0, rhs }.into()
    }
}

impl<Lhs, Rhs> std::ops::Rem<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Modulus<Lhs, Rhs>>;

    fn rem(self, rhs: Rhs) -> Self::Output {
        Modulus { lhs: self.0, rhs }.into()
    }
}

impl<Lhs> GeneratorWrapper<Lhs>
where
    Lhs: Generator,
{
    pub fn min<Rhs>(self, rhs: Rhs) -> GeneratorWrapper<Min<Lhs, Rhs>>
    where
        Rhs: Hybrid,
    {
        Min { lhs: self.0, rhs }.into()
    }

    pub fn max<Rhs>(self, rhs: Rhs) -> GeneratorWrapper<Max<Lhs, Rhs>>
    where
        Rhs: Hybrid,
    {
        Max { lhs: self.0, rhs }.into()
    }

    pub fn min_smooth<Rhs, S>(
        self,
        rhs: Rhs,
        smoothness: S,
    ) -> GeneratorWrapper<MinSmooth<Lhs, Rhs, S>>
    where
        Rhs: Hybrid,
        S: Hybrid,
    {
        MinSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        }
        .into()
    }

    pub fn max_smooth<Rhs, S>(
        self,
        rhs: Rhs,
        smoothness: S,
    ) -> GeneratorWrapper<MaxSmooth<Lhs, Rhs, S>>
    where
        Rhs: Hybrid,
        S: Hybrid,
    {
        MaxSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        }
        .into()
    }
}

impl<A> GeneratorWrapper<A>
where
    A: Generator,
{
    /// Fades between two generators with default min/max (-1.0, 1.0) and linear interpolation.
    pub fn fade<B, F>(self, b: B, fade: F) -> GeneratorWrapper<Fade<A, B, F, f32, f32>>
    where
        B: Generator,
        F: Hybrid,
    {
        Fade {
            a: self.0,
            b,
            fade,
            fade_min: -1.0,
            fade_max: 1.0,
            interpolation: FadeInterpolation::default(),
        }
        .into()
    }

    /// Fades between two generators with custom min/max range and interpolation.
    pub fn fade_with_range<B, F, FMin, FMax>(
        self,
        b: B,
        fade: F,
        fade_min: FMin,
        fade_max: FMax,
        interpolation: FadeInterpolation,
    ) -> GeneratorWrapper<Fade<A, B, F, FMin, FMax>>
    where
        B: Generator,
        F: Hybrid,
        FMin: Hybrid,
        FMax: Hybrid,
    {
        Fade {
            a: self.0,
            b,
            fade,
            fade_min,
            fade_max,
            interpolation,
        }
        .into()
    }
}

impl<V> GeneratorWrapper<V>
where
    V: Hybrid,
{
    pub fn powf<P>(self, pow: P) -> GeneratorWrapper<PowFloat<V, P>>
    where
        P: Hybrid,
    {
        PowFloat { value: self.0, pow }.into()
    }
}

impl<V> GeneratorWrapper<V>
where
    V: Generator,
{
    pub fn powi(self, pow: i32) -> GeneratorWrapper<PowInt<V>> {
        PowInt { value: self.0, pow }.into()
    }
}

impl<T> GeneratorWrapper<T>
where
    T: Hybrid,
{
    pub fn recip(self) -> GeneratorWrapper<Divide<f32, T>> {
        Divide {
            lhs: 1.0,
            rhs: self.0,
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        generator::{basic::constant, perlin::perlin, simplex::simplex, value::value},
        test_utils::*,
    };

    #[test]
    fn test_add() {
        let node = (perlin() + 0.5).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_subtract() {
        let node = (perlin() - simplex()).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_multiply() {
        let node = (perlin() * 2.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_divide() {
        let node = (perlin() / 2.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_modulus() {
        let node = (perlin() % 0.5).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_min() {
        let node = perlin().min(0.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_max() {
        let node = perlin().max(0.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_negate() {
        let node = (-perlin()).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_min_smooth() {
        let node = perlin().min_smooth(simplex(), 0.1).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_max_smooth() {
        let node = perlin().max_smooth(simplex(), 0.1).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_fade() {
        let node = perlin().fade(simplex(), 0.5).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_fade_with_range() {
        let node = perlin()
            .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Hermite)
            .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_powf() {
        let node = perlin().abs().powf(2.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_powi() {
        let node = perlin().powi(2).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_recip() {
        // recip() creates 1.0 / value
        let node = (constant(2.0) + constant(0.0)).recip().build();
        let mut output = [0.0f32; 4];
        let min_max = node
            .0
            .gen_uniform_grid_2d(&mut output, 0.0, 0.0, 2, 2, 0.1, 0.1, 1337);
        assert!(min_max.min.is_finite());
        assert!(min_max.max.is_finite());
        // 1/2 = 0.5
        assert!(output.iter().all(|&v| (v - 0.5).abs() < 0.001));
    }

    #[test]
    fn test_fade_interpolation_linear() {
        let node = perlin()
            .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Linear)
            .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_fade_interpolation_quintic() {
        let node = perlin()
            .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Quintic)
            .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_hybrid_fade() {
        // Using a generator for fade value
        let fade_gen = simplex();
        let node = perlin().fade(value(), fade_gen).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_hybrid_min_smooth() {
        // Using a generator for smoothness
        let smooth_gen = simplex().remap(-1.0, 1.0, 0.05, 0.2);
        let node = perlin().min_smooth(value(), smooth_gen).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_hybrid_powf() {
        // Using a generator for power
        let pow_gen = simplex().remap(-1.0, 1.0, 1.5, 2.5);
        let node = perlin().abs().powf(pow_gen).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_generator_add_generator() {
        let node = (perlin() + simplex()).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_generator_multiply_generator() {
        let node = (perlin() * simplex()).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_generator_min_generator() {
        let node = perlin().min(simplex()).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_generator_max_generator() {
        let node = perlin().max(simplex()).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_param_fade_min_max() {
        let node1 = perlin()
            .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Linear)
            .build();
        let node2 = perlin()
            .fade_with_range(simplex(), 0.5, 0.0, 0.5, FadeInterpolation::Linear)
            .build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "Fade.Fade Min/Max");
    }

    #[test]
    fn test_param_fade_interpolation() {
        let node1 = perlin()
            .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Linear)
            .build();
        let node2 = perlin()
            .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Quintic)
            .build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "Fade.Interpolation");
    }

    #[test]
    fn test_param_pow_float() {
        let node1 = perlin().abs().powf(1.5).build();
        let node2 = perlin().abs().powf(3.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "PowFloat.Pow");
    }

    #[test]
    fn test_param_pow_int() {
        let node1 = perlin().powi(2).build();
        let node2 = perlin().powi(3).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "PowInt.Pow");
    }
}
