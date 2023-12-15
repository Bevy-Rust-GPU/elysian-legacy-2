use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{BoundingError, ContextGet, ContextSet, Distance, Position, Support},
    fn_traits::Func1,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeriveBoundingError {}

impl DeriveBoundingError {
    pub fn field<CA, CB, CC>(f: impl Func1<CA, CB>) -> impl Func1<CA, CC>
    where
        CA: Clone + ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
        CB: ContextGet<Distance<f32>>
            + ContextGet<Support<Vec2>>
            + ContextSet<BoundingError<f32>, Set = CC>,
    {
        move |ca: CA| {
            let Position(p) = ca.get();
            let cb = f(ca.clone());
            let Support(s) = cb.get();
            let cc = f(ca.set(Position(p + s)));
            let Distance(d) = cc.get();
            cb.set(BoundingError(d))
        }
    }
}
