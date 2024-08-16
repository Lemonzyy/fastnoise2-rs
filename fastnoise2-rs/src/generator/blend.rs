use crate::FastNoise;

use super::{Hybrid, Node, NodeWrapper};

#[derive(Clone, Copy)]
pub struct Add<LHS: Node, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy)]
pub struct Subtract<LHS: Hybrid, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy)]
pub struct Multiply<LHS: Node, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy)]
pub struct Divide<LHS: Hybrid, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy)]
pub struct Min<LHS: Node, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy)]
pub struct Max<LHS: Node, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy)]
pub struct MinSmooth<LHS: Node, RHS: Hybrid, SMOOTHNESS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
    pub smoothness: SMOOTHNESS,
}

#[derive(Clone, Copy)]
pub struct MaxSmooth<LHS: Node, RHS: Hybrid, SMOOTHNESS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
    pub smoothness: SMOOTHNESS,
}

#[derive(Clone, Copy)]
pub struct Fade<A: Node, B: Node, FADE: Hybrid> {
    pub a: A,
    pub b: B,
    pub fade: FADE,
}

#[derive(Clone, Copy)]
pub struct PowFloat<VALUE: Hybrid, POW: Hybrid> {
    pub value: VALUE,
    pub pow: POW,
}

#[derive(Clone, Copy)]
pub struct PowInt<VALUE: Node> {
    pub value: VALUE,
    pub pow: i32,
}

impl<LHS: Node, RHS: Hybrid> Node for Add<LHS, RHS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Add").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node
    }
}

impl<LHS: Hybrid, RHS: Hybrid> Node for Subtract<LHS, RHS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Subtract").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node
    }
}

impl<LHS: Node, RHS: Hybrid> Node for Multiply<LHS, RHS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Multiply").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node
    }
}

impl<LHS: Hybrid, RHS: Hybrid> Node for Divide<LHS, RHS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Divide").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node
    }
}

impl<LHS: Node, RHS: Hybrid> Node for Min<LHS, RHS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Min").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node
    }
}

impl<LHS: Node, RHS: Hybrid> Node for Max<LHS, RHS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Max").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node
    }
}

impl<LHS: Node, RHS: Hybrid, SMOOTHNESS: Hybrid> Node for MinSmooth<LHS, RHS, SMOOTHNESS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("MinSmooth").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        node
    }
}

impl<LHS: Node, RHS: Hybrid, SMOOTHNESS: Hybrid> Node for MaxSmooth<LHS, RHS, SMOOTHNESS> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("MaxSmooth").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        node
    }
}

impl<A: Node, B: Node, FADE: Hybrid> Node for Fade<A, B, FADE> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Fade").unwrap();
        node.set("A", self.a).unwrap();
        node.set("B", self.b).unwrap();
        node.set("Fade", self.fade).unwrap();
        node
    }
}

impl<VALUE: Hybrid, POW: Hybrid> Node for PowFloat<VALUE, POW> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("PowFloat").unwrap();
        node.set("Value", self.value).unwrap();
        node.set("Pow", self.pow).unwrap();
        node
    }
}

impl<VALUE: Node> Node for PowInt<VALUE> {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("PowInt").unwrap();
        node.set("Value", self.value).unwrap();
        node.set("Pow", self.pow).unwrap();
        node
    }
}

// TODO
// impl<Lhs: Node, Rhs: Node> std::ops::Add<Rhs> for Lhs {
//     type Output = Add<Lhs, Rhs>;

//     fn add(self, rhs: Self) -> Self::Output {
//         Add { lhs: self, rhs }
//     }
// }

// impl<Lhs: Node, Rhs: Hybrid> std::ops::Add<Rhs> for NodeWrapper<Lhs> {
//     type Output = Add<Lhs, Rhs>;

//     fn add(self, rhs: Rhs) -> Self::Output {
//         Add { lhs: self.0, rhs }
//     }
// }
