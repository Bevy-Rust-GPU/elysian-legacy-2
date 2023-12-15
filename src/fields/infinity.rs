use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextSet, Distance, Gradient, Uv},
    field_traits::{DistanceFunction, Field, GradientFunction, UvFunction},
    fn_traits::Func1,
    lifting::lift_replace_1,
};

/// Infinite field
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Infinity<const D: usize> {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), (), CA, CD> for Infinity<2>
where
    CA: ContextSet<Distance<f32>, Set = CB>,
    CB: ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field(t: ()) -> impl Func1<CA, CD> {
        compose!(
            lift_replace_1(Self::dist),
            lift_replace_1(Self::grad),
            lift_replace_1(Self::uv),
        )
    }
}

impl DistanceFunction<()> for Infinity<2> {
    fn dist(_: ()) -> Distance<f32> {
        Distance(f32::INFINITY)
    }
}

impl GradientFunction<()> for Infinity<2> {
    fn grad(_: ()) -> Gradient<Vec2> {
        Gradient(Vec2::ZERO)
    }
}

impl UvFunction<()> for Infinity<2> {
    fn uv(_: ()) -> Uv<Vec2> {
        Uv(Vec2::ZERO)
    }
}
