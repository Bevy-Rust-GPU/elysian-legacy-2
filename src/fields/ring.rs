use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::Field,
    fields::circle::Circle,
    fn_traits::Func1,
    modifiers::isomanifold::Isomanifold,
};

pub enum Ring {}

impl<F, CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), (f32, f32, F), CA, CD>
    for Ring
where
    F: Func1<f32, f32>,
    CA: ContextGet<Position<Vec2>> + ContextSet<Distance<f32>, Set = CB>,
    CB: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextGet<Distance<f32>> + ContextSet<Uv<Vec2>, Set = CD>,
    CD: ContextGet<Distance<f32>>
        + ContextSet<Distance<f32>, Set = CD>
        + ContextGet<Gradient<Vec2>>
        + ContextSet<Gradient<Vec2>, Set = CD>
        + ContextGet<Uv<Vec2>>
        + ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field((radius, width, k): (f32, f32, F)) -> impl Func1<CA, CD> {
        compose!(Circle::field(radius), Isomanifold::field(width, k))
    }
}
