use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::{DistanceFunction, Field, GradientFunction, UvClosure},
    fields::point::Point,
    fn_traits::Func1,
    lifting::lift_modify,
    modifiers::elongate_axes::ElongateAxes,
};

pub enum Quad {}

impl<CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), Vec2, CA, CD> for Quad
where
    CA: ContextGet<Position<Vec2>> + ContextSet<Uv<Vec2>, Set = CB>,
    CB: ContextGet<Position<Vec2>>
        + ContextSet<Position<Vec2>, Set = CB>
        + ContextSet<Distance<f32>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CD>,
    CD: ContextGet<Distance<f32>> + ContextSet<Distance<f32>, Set = CD>,
{
    fn field(size: Vec2) -> impl Func1<CA, CD> {
        compose!(
            lift_modify(Self::uv(size)),
            ElongateAxes::field(
                size,
                compose!(lift_modify(Point::dist), lift_modify(Point::grad),)
            ),
        )
    }
}

impl UvClosure<Vec2, Position<Vec2>> for Quad {
    fn uv(size: Vec2) -> impl Func1<Position<Vec2>, Uv<Vec2>> {
        move |Position(p)| Uv(p / size)
    }
}
