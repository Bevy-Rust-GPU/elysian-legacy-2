use rust_gpu_bridge::{glam::Vec2, Sin, Cos};

use crate::{context::{ContextGet, Position, ContextSet, Gradient}, compose, lifting::lift_modify, field_traits::{PositionClosure, GradientClosure}, fn_traits::Func1};

pub enum Rotate<const D: usize> {}

impl Rotate<2> {
    pub fn field<CA, CB>(angle: f32, f: impl Func1<CA, CB>) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
        CB: ContextGet<Gradient<Vec2>> + ContextSet<Gradient<Vec2>, Set = CB>,
    {
        compose!(
            lift_modify(Self::pos(angle)),
            f,
            lift_modify(Self::grad(angle))
        )
    }
}

impl PositionClosure<f32, Position<Vec2>> for Rotate<2> {
    fn pos(angle: f32) -> impl Func1<Position<Vec2>, Position<Vec2>> {
        move |Position(p): Position<Vec2>| {
            let cs = angle.cos();
            let sn = angle.sin();
            Position(Vec2::new(p.x * cs - p.y * sn, p.x * sn + p.y * cs))
        }
    }
}

impl GradientClosure<f32, Gradient<Vec2>> for Rotate<2> {
    fn grad(angle: f32) -> impl Func1<Gradient<Vec2>, Gradient<Vec2>> {
        move |Gradient(g): Gradient<Vec2>| {
            let angle = -angle;
            let cs = angle.cos();
            let sn = angle.sin();
            Gradient(Vec2::new(g.x * cs - g.y * sn, g.x * sn + g.y * cs))
        }
    }
}

