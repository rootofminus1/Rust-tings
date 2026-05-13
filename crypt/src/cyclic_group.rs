use num_traits::{FromPrimitive, One, PrimInt, Zero};
use rand::distr::uniform::SampleUniform;


// pub type Scalar = u64;

pub trait CyclicGroup {
    type Element: Sized + Copy + PartialEq;
    type Scalar: PrimInt + SampleUniform + FromPrimitive;

    fn identity(&self) -> Self::Element;
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element;
    
    fn generator(&self) -> Self::Element;

    /// the order of the group
    fn generator_order(&self) -> Self::Scalar;

    /// applying a to itself n times
    fn scalar_mul(&self, n: Self::Scalar, a: Self::Element) -> Self::Element {
        let mut result = self.identity();
        let mut addend = a;
        let mut n = n;

        while n > Self::Scalar::zero() {
            if (n & Self::Scalar::one()) == Self::Scalar::one() {
                result = self.op(result, addend);
            }
            addend = self.op(addend, addend);
            n = n >> 1;
        }

        result
    }
}