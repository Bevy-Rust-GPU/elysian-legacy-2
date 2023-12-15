use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{
    context::{Color, ContextGet, ContextSet, Uv},
    field_traits::ColorFunction,
    fn_traits::Func1,
    lifting::lift_modify,
};

pub enum UvToColor {}

impl UvToColor {
    pub fn field<CA, CB>() -> impl Func1<CA, CB>
    where
        CA: ContextGet<Uv<Vec2>> + ContextSet<Color<Vec3>, Set = CB>,
    {
        lift_modify(Self::color)
    }
}

impl ColorFunction<Uv<Vec2>> for UvToColor {
    fn color(Uv(u): Uv<Vec2>) -> Color<Vec3> {
        Color(u.extend(0.0))
    }
}
