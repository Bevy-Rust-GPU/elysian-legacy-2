use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{
    context::{Color, ContextGet, ContextSet, Support},
    field_traits::ColorFunction,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportToColor {}

impl SupportToColor {
    pub fn field<CA, CB>() -> impl Func1<CA, CB>
    where
        CA: ContextGet<Support<Vec2>> + ContextSet<Color<Vec3>, Set = CB>,
    {
        lift_modify(Self::color)
    }
}

impl ColorFunction<Support<Vec2>> for SupportToColor {
    fn color(Support(s): Support<Vec2>) -> Color<Vec3> {
        Color((s * 0.5 + 0.5).extend(0.0))
    }
}

