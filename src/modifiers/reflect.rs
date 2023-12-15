use rust_gpu_bridge::{glam::Vec2, Reflect as ReflectTrait, Mix};

use crate::{context::{ContextGet, ContextSet, Position, Gradient, Uv}, fn_traits::Func1, compose, lifting::lift_modify};

pub enum Reflect<const D: usize> {}

impl Reflect<2> {
    pub fn dist<CA>(norm: Vec2) -> impl Func1<CA, CA>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
    {
        lift_modify(move |Position(p): Position<Vec2>| {
            let d = p.dot(norm);
            if d >= 0.0 {
                Position(p)
            } else {
                Position(p.reflect(norm))
            }
        })
    }

    pub fn grad<CA, CB>(
        norm: Vec2,
        k: impl Func1<f32, f32>,
        f: impl Func1<CA, CB>,
    ) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>>,
        CB: ContextGet<Gradient<Vec2>> + ContextSet<Gradient<Vec2>, Set = CB>,
    {
        move |ca: CA| {
            let Position(p) = ca.get();
            let cb = f(ca);
            let Gradient(g) = cb.get();
            let d = p.dot(norm);
            let k = k(d);
            let t = ((d / k) * 0.5 + 0.5).clamp(0.0, 1.0);
            cb.set(Gradient(g.reflect(norm).mix(g, Vec2::splat(t))))
        }
    }

    pub fn uv<CA, CB>(
        norm: Vec2,
        k: impl Func1<f32, f32>,
        f: impl Func1<CA, CB>,
    ) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>>,
        CB: ContextGet<Uv<Vec2>> + ContextSet<Uv<Vec2>, Set = CB>,
    {
        move |ca: CA| {
            let Position(p) = ca.get();
            let cb = f(ca);
            let Uv(u) = cb.get();
            let d = p.dot(norm);
            let k = k(d);
            let t = ((d / k) * 0.5 + 0.5).clamp(0.0, 1.0);
            cb.set(Uv(u.reflect(norm).mix(u, Vec2::splat(t))))
        }
    }

    pub fn field<CA, CB>(
        norm: Vec2,
        k: impl Func1<f32, f32>,
        f: impl Func1<CA, CB>,
    ) -> impl Func1<CA, CB>
    where
        CA: ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CA>,
        CB: ContextGet<Position<Vec2>>
            + ContextGet<Gradient<Vec2>>
            + ContextSet<Gradient<Vec2>, Set = CB>
            + ContextGet<Uv<Vec2>>
            + ContextSet<Uv<Vec2>, Set = CB>,
    {
        Self::uv(norm, k, Self::grad(norm, k, compose!(Self::dist(norm), f)))
    }
}

