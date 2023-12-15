use rust_gpu_bridge::glam::Vec3;

use crate::{
    context::{Color, ContextGet, ContextSet, Light},
    field_traits::ColorFunction,
    fn_traits::Func1,
    lifting::lift_modify,
};

pub enum LightToColor {}

impl LightToColor {
    pub fn field<CA, CB>() -> impl Func1<CA, CB>
    where
        CA: ContextGet<Light<f32>> + ContextSet<Color<Vec3>, Set = CB>,
    {
        lift_modify(Self::color)
    }
}

impl ColorFunction<Light<f32>> for LightToColor {
    fn color(Light(l): Light<f32>) -> Color<Vec3> {
        Color(Vec3::splat(l))
    }
}
