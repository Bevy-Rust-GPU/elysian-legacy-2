use rust_gpu_bridge::{glam::Vec2, Cos, Sin, Sqrt};

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Gradient, Position, Uv},
    field_traits::Field,
    fn_traits::Func1,
    modifiers::{demanifold::Demanifold, reflect::Reflect, translate::Translate},
};

use super::line::Line;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Triangle {}

impl<F, CA, CB, CC, CD> Field<(Distance<f32>, Gradient<Vec2>, Uv<Vec2>), (f32, F), CA, CD>
    for Triangle
where
    F: Func1<f32, f32>,
    CA: ContextGet<Position<Vec2>>
        + ContextSet<Position<Vec2>, Set = CA>
        + ContextSet<Distance<f32>, Set = CB>,
    CB: ContextGet<Position<Vec2>> + ContextSet<Gradient<Vec2>, Set = CC>,
    CC: ContextGet<Position<Vec2>> + ContextGet<Distance<f32>> + ContextSet<Uv<Vec2>, Set = CD>,
    CD: ContextGet<Position<Vec2>>
        + ContextGet<Distance<f32>>
        + ContextSet<Distance<f32>, Set = CD>
        + ContextGet<Gradient<Vec2>>
        + ContextSet<Gradient<Vec2>, Set = CD>
        + ContextGet<Uv<Vec2>>
        + ContextSet<Uv<Vec2>, Set = CD>,
{
    fn field((radius, k): (f32, F)) -> impl Func1<CA, CD> {
        let angle = core::f32::consts::TAU / 3.0;
        let sqrt_3 = 3.0_f32.sqrt();
        let width = radius * core::f32::consts::FRAC_PI_3.sin();

        Reflect::field(
            Vec2::X,
            k,
            Reflect::field(
                Vec2::new(angle.cos(), angle.sin()),
                k,
                compose!(
                    Translate::field(Vec2::Y * width / sqrt_3,),
                    Demanifold::field(Vec2::Y, Line::field(Vec2::X)),
                ),
            ),
        )
    }
}
