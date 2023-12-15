use rust_gpu_bridge::{glam::Vec2, Abs, Sign};

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::{DistanceFunction, Field, GradientFunction, UvFunction},
    fn_traits::Func1,
    lifting::lift_modify,
};

/// Taxicab field
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Taxicab<const D: usize> {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), (), CA, CD> for Taxicab<2>
where
    CA: ContextGet<Position<Vec2>> + ContextSet<Distance<f32>, Set = CB>,
    CB: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field(_: ()) -> impl Func1<CA, CD> {
        compose!(
            lift_modify(Self::dist),
            lift_modify(Self::grad),
            lift_modify(Self::uv)
        )
    }
}

impl DistanceFunction<Position<Vec2>> for Taxicab<2> {
    fn dist(Position(p): Position<Vec2>) -> Distance<f32> {
        Distance(p.x.abs() + p.y.abs())
    }
}

impl GradientFunction<Position<Vec2>> for Taxicab<2> {
    fn grad(Position(p): Position<Vec2>) -> Gradient<Vec2> {
        Gradient(Vec2::new(p.x.sign(), p.y.sign()))
    }
}

impl UvFunction<Position<Vec2>> for Taxicab<2> {
    fn uv(Position(p): Position<Vec2>) -> Uv<Vec2> {
        Uv(p)
    }
}
