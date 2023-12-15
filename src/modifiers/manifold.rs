use rust_gpu_bridge::{glam::Vec2, Abs};

use crate::{
    context::{ContextGet, Distance, Gradient, Uv, ContextSet},
    fn_traits::Func1, compose, lifting::lift_modify, field_traits::{DistanceFunction, GradientClosure},
};

pub enum Manifold<const D: usize> {}

impl Manifold<2> {
    pub fn field<CA>(k: impl Func1<f32, f32>) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Distance<f32>>
            + ContextSet<Distance<f32>, Set = CA>
            + ContextGet<Gradient<Vec2>>
            + ContextSet<Gradient<Vec2>, Set = CA>
            + ContextGet<Uv<Vec2>>
            + ContextSet<Uv<Vec2>, Set = CA>,
    {
        compose!(lift_modify(Self::grad(k)), lift_modify(Self::dist))
    }
}

impl DistanceFunction<Distance<f32>> for Manifold<2> {
    fn dist(Distance(d): Distance<f32>) -> Distance<f32> {
        Distance(d.abs())
    }
}

impl<F> GradientClosure<F, (Distance<f32>, Gradient<Vec2>)> for Manifold<2>
where
    F: Func1<f32, f32>,
{
    fn grad(k: F) -> impl Func1<(Distance<f32>, Gradient<Vec2>), Gradient<Vec2>> {
        move |(Distance(d), Gradient(g)): (Distance<f32>, Gradient<Vec2>)| {
            Gradient(g * (d / k(d)).clamp(-1.0, 1.0))
        }
    }
}

