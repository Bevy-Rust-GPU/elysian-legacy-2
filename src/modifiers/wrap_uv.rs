use rust_gpu_bridge::{glam::Vec2, Mod};

use crate::{
    context::{ContextGet, ContextSet, Uv},
    field_traits::UvClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WrapUv {}

impl WrapUv {
    pub fn field<CA>(period: Vec2) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Uv<Vec2>> + ContextSet<Uv<Vec2>, Set = CA>,
    {
        lift_modify(Self::uv(period))
    }
}

impl UvClosure<Vec2, Uv<Vec2>> for WrapUv {
    fn uv(period: Vec2) -> impl Func1<Uv<Vec2>, Uv<Vec2>> {
        move |Uv(u): Uv<Vec2>| {
            Uv(Vec2::new(
                u.x.modulo(period.x),
                u.y.modulo(period.y),
            ))
        }
    }
}
