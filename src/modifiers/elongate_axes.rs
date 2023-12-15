use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{ContextGet, ContextSet, Distance, Position},
    fn_traits::Func1,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElongateAxes<const D: usize> {}

impl ElongateAxes<2> {
    pub fn field<CA, CB>(dir: Vec2, f: impl Func1<CA, CB>) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
        CB: ContextGet<Distance<f32>> + ContextSet<Distance<f32>, Set = CB>,
    {
        move |ca: CA| {
            let Position(p) = ca.get();
            let q = p.abs() - dir;
            let cb = f(ca.set(Position(Vec2::new(q.x.max(0.0), q.y.max(0.0)))));
            let Distance(d) = cb.get();
            cb.set(Distance(d + q.x.max(q.y).min(0.0)))
        }
    }
}
