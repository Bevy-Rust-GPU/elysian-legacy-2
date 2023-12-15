use rust_gpu_bridge::{glam::Vec2, Sign};

use crate::{context::{ContextGet, Position, Distance, ContextSet, Gradient}, compose, fn_traits::Func1, field_traits::GradientClosure, lifting::lift_modify};

pub enum Demanifold<const D: usize> {}

impl Demanifold<2> {
    pub fn dist<CA, CB>(dir: Vec2, f: impl Func1<CA, CB>) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>>,
        CB: ContextGet<Distance<f32>> + ContextSet<Distance<f32>, Set = CB>,
    {
        move |ca: CA| {
            let Position(p) = ca.get();
            let cb = f(ca);
            let Distance(d) = cb.get();
            cb.set(Distance(d * p.dot(dir).sign()))
        }
    }

    pub fn field<CA, CB>(dir: Vec2, f: impl Func1<CA, CB>) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>>,
        CB: ContextGet<Distance<f32>>
            + ContextSet<Distance<f32>, Set = CB>
            + ContextSet<Gradient<Vec2>, Set = CB>,
    {
        compose!(Self::dist(dir, f), lift_modify(Self::grad(dir)))
    }
}

impl GradientClosure<Vec2, ()> for Demanifold<2> {
    fn grad(dir: Vec2) -> impl Func1<(), Gradient<Vec2>> {
        move |_| Gradient(dir)
    }
}

