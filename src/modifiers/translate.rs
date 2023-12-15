use rust_gpu_bridge::glam::Vec2;

use crate::{fn_traits::Func1, context::{ContextGet, ContextSet, Position}, lifting::lift_modify, field_traits::PositionClosure};

/// Translate input
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Translate<const N: usize> {}

impl Translate<2> {
    pub fn field<CA>(delta: Vec2) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
    {
        lift_modify(Self::pos(delta))
    }
}

impl PositionClosure<Vec2, Position<Vec2>> for Translate<2> {
    fn pos(delta: Vec2) -> impl Func1<Position<Vec2>, Position<Vec2>> {
        move |Position(p)| Position(p - delta)
    }
}

