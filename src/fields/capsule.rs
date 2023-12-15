use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::Field,
    fields::line::Line,
    fn_traits::Func1,
    modifiers::isosurface::Isosurface,
};

pub enum Capsule<const D: usize> {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), (Vec2, f32), CA, CD>
    for Capsule<2>
where
    CA: ContextGet<Position<Vec2>>
        + ContextSet<Position<Vec2>, Set = CA>
        + ContextSet<Distance<f32>, Set = CB>,
    CB: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextGet<Distance<f32>> + ContextSet<Uv<Vec2>, Set = CD>,
    CD: ContextGet<Distance<f32>>
        + ContextSet<Distance<f32>, Set = CD>
        + ContextGet<Uv<Vec2>>
        + ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field((dir, radius): (Vec2, f32)) -> impl Func1<CA, CD> {
        compose!(Line::field(dir), Isosurface::field(radius))
    }
}
