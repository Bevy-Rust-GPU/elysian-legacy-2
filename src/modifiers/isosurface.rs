use rust_gpu_bridge::glam::Vec2;

use crate::{context::{ContextGet, Distance, ContextSet, Uv}, compose, lifting::lift_modify, field_traits::{DistanceClosure, UvClosure}, fn_traits::Func1};

pub enum Isosurface<const D: usize> {}

impl Isosurface<2> {
    pub fn field<CA>(dist: f32) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Distance<f32>>
            + ContextSet<Distance<f32>, Set = CA>
            + ContextGet<Uv<Vec2>>
            + ContextSet<Uv<Vec2>, Set = CA>,
    {
        compose!(
            lift_modify(Self::dist(dist)),
            lift_modify(Self::uv(dist))
        )
    }
}

impl DistanceClosure<f32, Distance<f32>> for Isosurface<2> {
    fn dist(dist: f32) -> impl Func1<Distance<f32>, Distance<f32>> {
        move |Distance(d)| Distance(d - dist)
    }
}

impl UvClosure<f32, Uv<Vec2>> for Isosurface<2> {
    fn uv(dist: f32) -> impl Func1<Uv<Vec2>, Uv<Vec2>> {
        move |Uv(u): Uv<Vec2>| Uv(Vec2::new(u.x, u.y - dist))
    }
}

