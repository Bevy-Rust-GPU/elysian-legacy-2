//! Gradient domain

use core::ops::{Mul, Neg};

use crate::LiftParam;

// Gradient domain
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gradient<T>(pub T);

impl<T, U> Mul<U> for Gradient<T>
where
    T: Mul<U>,
{
    type Output = Gradient<T::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        Gradient(self.0.mul(rhs))
    }
}

impl<T> Neg for Gradient<T>
where
    T: Neg,
{
    type Output = Gradient<T::Output>;

    fn neg(self) -> Self::Output {
        Gradient(self.0.neg())
    }
}

impl<T, D> LiftParam<D> for Gradient<T> {
    type LiftParam = Self;

    fn lift_param(self, _: D) -> Self::LiftParam {
        self
    }
}
