use crate::{
    context::{ContextGet, ContextSet, Light},
    field_traits::LightClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AmbientLight {}

impl AmbientLight {
    pub fn field<CA>(ambient: f32) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Light<f32>> + ContextSet<Light<f32>, Set = CA>,
    {
        lift_modify(Self::light(ambient))
    }
}

impl LightClosure<f32, Light<f32>> for AmbientLight {
    fn light(ambient: f32) -> impl Func1<Light<f32>, Light<f32>> {
        move |Light(l): Light<f32>| Light(l + ambient)
    }
}

