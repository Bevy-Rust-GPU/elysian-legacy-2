use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::{DistanceClosure, DistanceFunction, Field, GradientFunction, UvFunction},
    fn_traits::Func1,
    lifting::lift_modify,
    modifiers::isosurface::Isosurface,
};

use super::point::Point;

pub enum Circle {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), f32, CA, CD> for Circle
where
    CA: ContextGet<Position<Vec2>> + ContextSet<Distance<f32>, Set = CB>,
    CB: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextGet<Distance<f32>> + ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field(radius: f32) -> impl Func1<CA, CD> {
        compose!(
            lift_modify(Self::dist(radius)),
            lift_modify(Self::grad),
            lift_modify(Self::uv)
        )
    }
}

impl DistanceClosure<f32, Position<Vec2>> for Circle {
    fn dist(radius: f32) -> impl Func1<Position<Vec2>, Distance<f32>> {
        compose!(Point::dist, Isosurface::dist(radius))
    }
}

impl GradientFunction<Position<Vec2>> for Circle {
    fn grad(i: Position<Vec2>) -> Gradient<Vec2> {
        Point::grad(i)
    }
}

impl UvFunction<(Position<Vec2>, Distance<f32>)> for Circle {
    fn uv(i: (Position<Vec2>, Distance<f32>)) -> Uv<Vec2> {
        Point::uv(i)
    }
}
