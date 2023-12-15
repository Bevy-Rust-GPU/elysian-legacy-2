use rust_gpu_bridge::{glam::Vec2, Abs, Sign};

use crate::{
    context::{ContextGet, ContextSet, Position},
    field_traits::PositionClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Elongate<const D: usize> {}

impl Elongate<2> {
    pub fn field<CA>(dir: Vec2) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
    {
        lift_modify(Self::pos(dir))
    }
}

impl PositionClosure<Vec2, Position<Vec2>> for Elongate<2> {
    fn pos(dir: Vec2) -> impl Func1<Position<Vec2>, Position<Vec2>> {
        move |Position(p): Position<Vec2>| {
            let l = dir.length();
            let n = dir.normalize_or_zero();
            let dp = n.dot(p);
            let ds = dp.sign();
            let da = dp.abs();
            let d = da.min(l) * ds;

            Position(p - n * d)
        }
    }
}
