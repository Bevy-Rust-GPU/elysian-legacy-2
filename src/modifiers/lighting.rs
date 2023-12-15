use rust_gpu_bridge::glam::Vec3;

use crate::{
    context::{Color, Light, ContextGet, ContextSet},
    field_traits::ColorFunction,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lighting {}

impl Lighting {
    pub fn field<CA>() -> impl Func1<CA, CA>
    where
        CA: ContextGet<Color<Vec3>> + ContextGet<Light<f32>> + ContextSet<Color<Vec3>, Set = CA>,
    {
        lift_modify(Self::color)
    }
}

impl ColorFunction<(Color<Vec3>, Light<f32>)> for Lighting {
    fn color((Color(c), Light(l)): (Color<Vec3>, Light<f32>)) -> Color<Vec3> {
        Color(c * l)
    }
}
