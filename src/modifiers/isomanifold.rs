use rust_gpu_bridge::glam::Vec2;

use crate::{
    context::{ContextGet, ContextSet, Distance, Gradient, Uv},
    fn_traits::Func1, compose, modifiers::{manifold::Manifold, isosurface::Isosurface},
};

pub enum Isomanifold {}

impl Isomanifold {
    pub fn field<CA>(dist: f32, k: impl Func1<f32, f32>) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Distance<f32>>
            + ContextSet<Distance<f32>, Set = CA>
            + ContextGet<Gradient<Vec2>>
            + ContextSet<Gradient<Vec2>, Set = CA>
            + ContextGet<Uv<Vec2>>
            + ContextSet<Uv<Vec2>, Set = CA>,
    {
        compose!(Manifold::field(k), Isosurface::field(dist))
    }
}

