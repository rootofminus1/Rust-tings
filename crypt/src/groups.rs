use crate::elliptic_curve::{EllipticCurve, Point};
use crate::cyclic_group::CyclicGroup;



pub struct MultiplicativeZp {
    pub p: u64,
    pub g: u64,
    pub g_order: u64  // ord(g) in Z_p*, its just p-1 if g is a primitive root
}

impl CyclicGroup for MultiplicativeZp {
    type Element = u64;
    type Scalar = u64;

    fn identity(&self) -> Self::Element {
        1
    }

    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        (a * b) % self.p
    }

    fn generator(&self) -> Self::Element {
        self.g
    }

    fn generator_order(&self) -> u64 {
        self.g_order
    }
}

#[derive(Debug, Clone)]
pub struct EcGroup {
    pub curve: EllipticCurve,
    pub generator: Point,
    pub generator_order: u64
}

impl CyclicGroup for EcGroup {
    type Element = Point;
    type Scalar = u64;
    
    fn identity(&self) -> Self::Element {
        Point::Infinity
    }
    
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        self.curve.add(a, b)
    }
    
    fn generator(&self) -> Self::Element {
        self.generator
    }
    
    fn generator_order(&self) -> Self::Scalar {
        self.generator_order
    }
}
