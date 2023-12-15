use rust_gpu_bridge::{glam::Vec3, Abs};

use crate::{
    context::{BoundingError, Color, ContextGet, ContextSet},
    field_traits::ColorFunction,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundingErrorToColor {}

impl BoundingErrorToColor {
    pub fn field<CA, CB>() -> impl Func1<CA, CB>
    where
        CA: ContextGet<BoundingError<f32>> + ContextSet<Color<Vec3>, Set = CB>,
    {
        lift_modify(Self::color)
    }
}

impl ColorFunction<BoundingError<f32>> for BoundingErrorToColor {
    fn color(BoundingError(e): BoundingError<f32>) -> Color<Vec3> {
        Color(
            if e > 0.0 {
                Vec3::X
            } else {
                Vec3::new(1.0, 1.0, 0.0)
            } * e.abs(),
        )
    }
}
