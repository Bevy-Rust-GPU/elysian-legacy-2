use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{ContextGet, ContextSet, Uv},
    field_traits::UvClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OffsetUv {}

impl OffsetUv {
    pub fn field<CA>(ofs: Vec2) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Uv<Vec2>> + ContextSet<Uv<Vec2>, Set = CA>,
    {
        lift_modify(Self::uv(ofs))
    }
}

impl UvClosure<Vec2, Uv<Vec2>> for OffsetUv {
    fn uv(ofs: Vec2) -> impl Func1<Uv<Vec2>, Uv<Vec2>> {
        move |Uv(u): Uv<Vec2>| Uv(u + ofs)
    }
}

