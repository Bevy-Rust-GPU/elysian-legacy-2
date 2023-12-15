use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextSet, Distance, Gradient, Uv},
    field_traits::{DistanceFunction, Field, GradientFunction, UvFunction},
    fields::infinity::Infinity,
    fn_traits::Func1,
    lifting::lift_replace_1,
};

// Negative infinite field
pub enum NegInfinity<const D: usize> {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), (), CA, CD> for NegInfinity<2>
where
    CA: ContextSet<Distance<f32>, Set = CB>,
    CB: ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field(_: ()) -> impl Func1<CA, CD> {
        compose!(
            lift_replace_1(Self::dist),
            lift_replace_1(Infinity::grad),
            lift_replace_1(Infinity::uv),
        )
    }
}

impl DistanceFunction<()> for NegInfinity<2> {
    fn dist(_: ()) -> Distance<f32> {
        Distance(f32::NEG_INFINITY)
    }
}
