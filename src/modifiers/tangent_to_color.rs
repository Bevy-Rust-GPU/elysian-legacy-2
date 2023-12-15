use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs, Mix,
};

use crate::{
    context::{Color, ContextGet, ContextSet, Distance, Tangent},
    field_traits::ColorClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TangentToColor {}

impl TangentToColor {
    pub fn field<FK, CA, CB>(fk: FK) -> impl Func1<CA, CB>
    where
        FK: Func1<f32, f32>,
        CA: ContextGet<Distance<f32>>
            + ContextGet<Tangent<Vec2>>
            + ContextSet<Color<Vec3>, Set = CB>,
    {
        lift_modify(Self::color(fk))
    }
}

impl<F> ColorClosure<F, (Distance<f32>, Tangent<Vec2>)> for TangentToColor
where
    F: Func1<f32, f32>,
{
    fn color(fk: F) -> impl Func1<(Distance<f32>, Tangent<Vec2>), Color<Vec3>> {
        move |(Distance(d), Tangent(t)): (Distance<f32>, Tangent<Vec2>)| {
            let k = fk(d);
            let b = 0.0.mix(1.0 - d.abs(), (d / k) * 0.5 + 0.5);
            Color((t * 0.5 + 0.5).extend(b))
        }
    }
}
