use rust_gpu_bridge::glam::Vec2;

use crate::{context::{ContextSet, Distance, Gradient, Position, ContextGet}, fn_traits::Func1};

use super::rotate::Rotate;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Twist {}

impl Twist {
    pub fn field<CA, CB>(fac: f32, f: impl Func1<CA, CB>) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
        CB: ContextGet<Distance<f32>>
            + ContextSet<Distance<f32>, Set = CB>
            + ContextGet<Gradient<Vec2>>
            + ContextSet<Gradient<Vec2>, Set = CB>,
    {
        move |ca: CA| {
            let Position(p) = ca.get();
            let angle = p.length() * fac;

            Rotate::field(angle, f)(ca)
        }
    }
}

