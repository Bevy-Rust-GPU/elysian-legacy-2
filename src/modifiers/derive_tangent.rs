use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{ContextGet, ContextSet, Gradient, Tangent},
    field_traits::TangentFunction,
    lifting::lift_modify, fn_traits::Func1,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeriveTangent<const D: usize> {}

impl DeriveTangent<2> {
    pub fn field<CA, CB>() -> impl Func1<CA, CB>
    where
        CA: ContextGet<Gradient<Vec2>> + ContextSet<Tangent<Vec2>, Set = CB>,
    {
        lift_modify(Self::tangent)
    }
}

impl TangentFunction<Gradient<Vec2>> for DeriveTangent<2> {
    fn tangent(Gradient(g): Gradient<Vec2>) -> Tangent<Vec2> {
        Tangent(Vec2::new(g.y, g.x))
    }
}

