use rust_gpu_bridge::glam::Vec2;

use crate::{fn_traits::Func1, context::{ContextGet, ContextSet, Uv}, lifting::lift_modify, field_traits::UvClosure};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaleUv {}

impl ScaleUv {
    pub fn field<CA>(fac: Vec2) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Uv<Vec2>> + ContextSet<Uv<Vec2>, Set = CA>,
    {
        lift_modify(Self::uv(fac))
    }
}

impl UvClosure<Vec2, Uv<Vec2>> for ScaleUv {
    fn uv(fac: Vec2) -> impl Func1<Uv<Vec2>, Uv<Vec2>> {
        move |Uv(u): Uv<Vec2>| Uv(u * fac)
    }
}

