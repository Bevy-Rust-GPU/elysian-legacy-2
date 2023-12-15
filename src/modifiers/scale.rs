use rust_gpu_bridge::glam::Vec2;

use crate::{
    compose,
    context::{ContextGet, ContextSet, Distance, Position},
    field_traits::{DistanceClosure, PositionClosure},
    fn_traits::Func1,
    lifting::lift_modify,
};

pub enum Scale<const N: usize> {}

impl Scale<2> {
    pub fn field<CA, CB>(fac: f32, f: impl Func1<CA, CB>) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
        CB: ContextGet<Distance<f32>> + ContextSet<Distance<f32>, Set = CB>,
    {
        compose!(lift_modify(Self::pos(fac)), f, lift_modify(Self::dist(fac)))
    }
}

impl PositionClosure<f32, Position<Vec2>> for Scale<2> {
    fn pos(fac: f32) -> impl Func1<Position<Vec2>, Position<Vec2>> {
        move |Position(p): Position<Vec2>| Position(p / fac)
    }
}

impl DistanceClosure<f32, Distance<f32>> for Scale<2> {
    fn dist(fac: f32) -> impl Func1<Distance<f32>, Distance<f32>> {
        move |Distance(d): Distance<f32>| Distance(d * fac)
    }
}
