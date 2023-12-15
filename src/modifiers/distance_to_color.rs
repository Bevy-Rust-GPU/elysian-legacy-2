use rust_gpu_bridge::{glam::Vec3, Abs};

use crate::{
    context::{Color, ContextGet, ContextSet, Distance},
    field_traits::ColorFunction,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DistanceToColor {}

impl DistanceToColor {
    pub fn field<CA, CB>() -> impl Func1<CA, CB>
    where
        CA: ContextGet<Distance<f32>> + ContextSet<Color<Vec3>, Set = CB>,
    {
        lift_modify(Self::color)
    }
}

impl ColorFunction<Distance<f32>> for DistanceToColor {
    fn color(Distance(d): Distance<f32>) -> Color<Vec3> {
        let c = if d >= 0.0 {
            Vec3::new(1.0, 1.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 1.0)
        };

        Color(c * d.abs())
    }
}

