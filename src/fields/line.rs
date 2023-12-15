use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::{
        DistanceClosure, DistanceFunction, Field, GradientClosure, GradientFunction,
        PositionClosure,
    },
    fields::point::Point,
    fn_traits::Func1,
    modifiers::elongate::Elongate,
};

pub enum Line<const D: usize> {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), Vec2, CA, CD> for Line<2>
where
    CA: ContextGet<Position<Vec2>>
        + ContextSet<Position<Vec2>, Set = CA>
        + ContextSet<Distance<f32>, Set = CB>,
    CB: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextGet<Distance<f32>> + ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field(dir: Vec2) -> impl Func1<CA, CD> {
        compose!(Elongate::field(dir), Point::field(()))
    }
}

impl DistanceClosure<Vec2, Position<Vec2>> for Line<2> {
    fn dist(dir: Vec2) -> impl Func1<Position<Vec2>, Distance<f32>> {
        compose!(Elongate::pos(dir), Point::dist)
    }
}

impl GradientClosure<Vec2, Position<Vec2>> for Line<2> {
    fn grad(dir: Vec2) -> impl Func1<Position<Vec2>, Gradient<Vec2>> {
        compose!(Elongate::pos(dir), Point::grad)
    }
}
