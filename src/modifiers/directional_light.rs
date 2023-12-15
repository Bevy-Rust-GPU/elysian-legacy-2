use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{
    context::{ContextGet, ContextSet, Gradient, Light},
    field_traits::LightClosure,
    fn_traits::Func1,
    lifting::lift_modify,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum DirectionalLight {}

impl DirectionalLight {
    pub fn field<CA>(dir: Vec3, lum: f32) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Gradient<Vec2>> + ContextGet<Light<f32>> + ContextSet<Light<f32>, Set = CA>,
    {
        lift_modify(Self::light((dir, lum)))
    }
}

impl LightClosure<(Vec3, f32), (Gradient<Vec2>, Light<f32>)> for DirectionalLight {
    fn light((dir, lum): (Vec3, f32)) -> impl Func1<(Gradient<Vec2>, Light<f32>), Light<f32>> {
        move |(Gradient(g), Light(l)): (Gradient<Vec2>, Light<f32>)| {
            let n = g.extend(1.0).normalize_or_zero();
            Light(l + n.dot(dir.normalize_or_zero()) * lum)
        }
    }
}
