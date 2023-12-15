use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{ContextGet, ContextSet, Position},
    field_traits::PositionClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectVerticalMinus {}

impl AspectVerticalMinus {
    pub fn field<C>(viewport: Vec2) -> impl Func1<C, C>
    where
        C: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = C>,
    {
        lift_modify(Self::pos(viewport))
    }
}

impl PositionClosure<Vec2, Position<Vec2>> for AspectVerticalMinus {
    fn pos(viewport: Vec2) -> impl Func1<Position<Vec2>, Position<Vec2>> {
        move |Position(p)| Position(p * Vec2::new(viewport.x / viewport.y, 1.0))
    }
}
