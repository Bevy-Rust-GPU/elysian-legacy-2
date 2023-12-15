use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{ContextGet, ContextSet, Position},
    field_traits::PositionClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

use super::{
    aspect_horizontal_plus::AspectHorizontalPlus, aspect_vertical_minus::AspectVerticalMinus,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectAdaptivePlus {}

impl AspectAdaptivePlus {
    pub fn field<CA>(viewport: Vec2) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
    {
        lift_modify(Self::pos(viewport))
    }
}

impl PositionClosure<Vec2, Position<Vec2>> for AspectAdaptivePlus {
    fn pos(viewport: Vec2) -> impl Func1<Position<Vec2>, Position<Vec2>> {
        move |p: Position<Vec2>| {
            if viewport.y > viewport.x {
                AspectHorizontalPlus::pos(viewport)(p)
            } else {
                AspectVerticalMinus::pos(viewport)(p)
            }
        }
    }
}
