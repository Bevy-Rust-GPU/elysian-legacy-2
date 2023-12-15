use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{ContextGet, ContextSet, Distance, Gradient, Support},
    field_traits::SupportFunction,
    fn_traits::Func1,
    lifting::lift_modify,
};

pub enum DeriveSupportFunction<const D: usize> {}

impl DeriveSupportFunction<2> {
    pub fn field<CA, CB>() -> impl Func1<CA, CB>
    where
        CA: ContextGet<Distance<f32>>
            + ContextGet<Gradient<Vec2>>
            + ContextSet<Support<Vec2>, Set = CB>,
    {
        lift_modify(Self::support)
    }
}

impl SupportFunction<(Distance<f32>, Gradient<Vec2>)> for DeriveSupportFunction<2> {
    fn support((Distance(d), Gradient(g)): (Distance<f32>, Gradient<Vec2>)) -> Support<Vec2> {
        Support(-g * d)
    }
}
