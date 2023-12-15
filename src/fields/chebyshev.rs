use rust_gpu_bridge::{glam::Vec2, Abs, Mix, Sign, Step};

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::{DistanceFunction, Field, GradientFunction, UvFunction},
    lifting::lift_modify,
};

/// Chebyshev field
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Chebyshev<const D: usize> {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), (), CA, CD> for Chebyshev<2>
where
    CA: ContextGet<Position<Vec2>> + ContextSet<Distance<f32>, Set = CB>,
    CB: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field(_: ()) -> impl crate::fn_traits::Func1<CA, CD> {
        compose!(
            lift_modify(Self::dist),
            lift_modify(Self::grad),
            lift_modify(Self::uv)
        )
    }
}

impl DistanceFunction<Position<Vec2>> for Chebyshev<2> {
    fn dist(Position(p): Position<Vec2>) -> Distance<f32> {
        Distance(p.x.abs().max(p.y.abs()))
    }
}

impl GradientFunction<Position<Vec2>> for Chebyshev<2> {
    fn grad(Position(p): Position<Vec2>) -> Gradient<Vec2> {
        let a = p.abs();
        let s = p.sign();
        let g = (Vec2::X * s.x).mix(Vec2::Y * s.y, Vec2::splat(a.x.step(a.y)));
        Gradient(g)
    }
}

impl UvFunction<Position<Vec2>> for Chebyshev<2> {
    fn uv(Position(p): Position<Vec2>) -> Uv<Vec2> {
        Uv(p)
    }
}
