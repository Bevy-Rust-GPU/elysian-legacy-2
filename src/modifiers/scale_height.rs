use crate::{context::{ContextGet, Distance, ContextSet}, lifting::lift_modify, field_traits::DistanceClosure, fn_traits::Func1};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaleHeight {}

impl ScaleHeight {
    pub fn field<CA>(fac: f32) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Distance<f32>> + ContextSet<Distance<f32>, Set = CA>,
    {
        lift_modify(Self::dist(fac))
    }
}

impl DistanceClosure<f32, Distance<f32>> for ScaleHeight {
    fn dist(fac: f32) -> impl Func1<Distance<f32>, Distance<f32>> {
        move |Distance(d): Distance<f32>| Distance(d * fac)
    }
}

