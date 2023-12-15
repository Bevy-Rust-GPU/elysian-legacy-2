use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Mix,
};

use crate::fn_traits::Func1;

pub trait ContextGet<T> {
    fn get(&self) -> T;
}

impl<T> ContextGet<()> for T {
    fn get(&self) -> () {
        ()
    }
}

impl<T, A, B> ContextGet<(A, B)> for T
where
    T: ContextGet<A> + ContextGet<B>,
{
    fn get(&self) -> (A, B) {
        (self.get(), self.get())
    }
}

pub trait ContextSet<T> {
    type Set;

    fn set(self, t: T) -> Self::Set;
}

pub type SetT<T, U> = <T as ContextSet<U>>::Set;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Set {}

impl Set {
    pub fn field<T, CA, CB>(t: T) -> impl Func1<CA, CB>
    where
        CA: ContextSet<T, Set = CB>,
        T: Copy,
    {
        move |ca: CA| ca.set(t)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position<T>(pub T);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distance<T>(pub T);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gradient<T>(pub T);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uv<T>(pub T);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tangent<T>(pub T);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Support<T>(pub T);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundingError<T>(pub T);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color<T>(pub T);

pub enum SetColor {}

impl SetColor {
    pub fn field<T, CA, CB>(t: T) -> impl Func1<CA, CB>
    where
        CA: ContextSet<Color<T>, Set = CB>,
        T: Copy,
    {
        Set::field(Color(t))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Light<T>(pub T);

impl<T> Mix for Distance<T>
where
    T: Mix,
{
    type T = T::T;
    fn mix(self, rhs: Self, t: Self::T) -> Self {
        Distance(self.0.mix(rhs.0, t))
    }
}

impl Mix for Gradient<Vec2> {
    type T = f32;
    fn mix(self, rhs: Self, t: Self::T) -> Self {
        Gradient(self.0.mix(rhs.0, Vec2::splat(t)))
    }
}

impl Mix for Uv<Vec2> {
    type T = f32;
    fn mix(self, rhs: Self, t: Self::T) -> Self {
        Uv(self.0.mix(rhs.0, Vec2::splat(t)))
    }
}

impl Mix for Color<Vec3> {
    type T = f32;
    fn mix(self, rhs: Self, t: Self::T) -> Self {
        Color(self.0.mix(rhs.0, Vec3::splat(t)))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context<P = (), D = (), G = (), U = (), T = (), S = (), E = (), C = (), L = ()> {
    pos: P,
    dist: D,
    grad: G,
    uv: U,
    tangent: T,
    support: S,
    error: E,
    color: C,
    light: L,
}

impl Default for Context<(), (), (), (), (), (), (), (), ()> {
    fn default() -> Self {
        Context {
            pos: (),
            dist: (),
            grad: (),
            uv: (),
            tangent: (),
            support: (),
            error: (),
            color: (),
            light: (),
        }
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Position<P>>
    for Context<Position<P>, D, G, U, T, S, E, C, L>
where
    P: Clone,
{
    fn get(&self) -> Position<P> {
        self.pos.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Distance<D>>
    for Context<P, Distance<D>, G, U, T, S, E, C, L>
where
    D: Clone,
{
    fn get(&self) -> Distance<D> {
        self.dist.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Gradient<G>>
    for Context<P, D, Gradient<G>, U, T, S, E, C, L>
where
    G: Clone,
{
    fn get(&self) -> Gradient<G> {
        self.grad.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Uv<U>> for Context<P, D, G, Uv<U>, T, S, E, C, L>
where
    U: Clone,
{
    fn get(&self) -> Uv<U> {
        self.uv.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Tangent<T>>
    for Context<P, D, G, U, Tangent<T>, S, E, C, L>
where
    T: Clone,
{
    fn get(&self) -> Tangent<T> {
        self.tangent.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Support<S>>
    for Context<P, D, G, U, T, Support<S>, E, C, L>
where
    S: Clone,
{
    fn get(&self) -> Support<S> {
        self.support.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<BoundingError<E>>
    for Context<P, D, G, U, T, S, BoundingError<E>, C, L>
where
    E: Clone,
{
    fn get(&self) -> BoundingError<E> {
        self.error.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Color<C>> for Context<P, D, G, U, T, S, E, Color<C>, L>
where
    C: Clone,
{
    fn get(&self) -> Color<C> {
        self.color.clone()
    }
}

impl<P, D, G, U, T, S, E, C, L> ContextGet<Light<L>> for Context<P, D, G, U, T, S, E, C, Light<L>>
where
    L: Clone,
{
    fn get(&self) -> Light<L> {
        self.light.clone()
    }
}

impl<PA, PB, D, G, U, T, S, E, C, L> ContextSet<Position<PB>>
    for Context<PA, D, G, U, T, S, E, C, L>
{
    type Set = Context<Position<PB>, D, G, U, T, S, E, C, L>;

    fn set(self, t: Position<PB>) -> Self::Set {
        let Context {
            pos: _,
            dist,
            grad,
            uv,
            tangent,
            support,
            error,
            color,
            light,
        } = self;

        Context {
            pos: t,
            dist,
            grad,
            uv,
            tangent,
            support,
            error,
            color,
            light,
        }
    }
}

impl<P, DA, DB, G, U, T, S, E, C, L> ContextSet<Distance<DB>>
    for Context<P, DA, G, U, T, S, E, C, L>
{
    type Set = Context<P, Distance<DB>, G, U, T, S, E, C, L>;

    fn set(self, t: Distance<DB>) -> Self::Set {
        let Context {
            pos,
            dist: _,
            grad,
            uv,
            tangent,
            support,
            error,
            color,
            light,
        } = self;

        Context {
            pos,
            dist: t,
            grad,
            uv,
            tangent,
            support,
            error,
            color,
            light,
        }
    }
}

impl<P, D, GA, GB, U, T, S, E, C, L> ContextSet<Gradient<GB>>
    for Context<P, D, GA, U, T, S, E, C, L>
{
    type Set = Context<P, D, Gradient<GB>, U, T, S, E, C, L>;

    fn set(self, t: Gradient<GB>) -> Self::Set {
        let Context {
            pos,
            dist,
            grad: _,
            uv,
            tangent,
            support,
            error,
            color,
            light,
        } = self;

        Context {
            pos,
            dist,
            grad: t,
            uv,
            tangent,
            support,
            error,
            color,
            light,
        }
    }
}

impl<P, D, G, UA, UB, T, S, E, C, L> ContextSet<Uv<UB>> for Context<P, D, G, UA, T, S, E, C, L> {
    type Set = Context<P, D, G, Uv<UB>, T, S, E, C, L>;

    fn set(self, t: Uv<UB>) -> Self::Set {
        let Context {
            pos,
            dist,
            grad,
            uv: _,
            tangent,
            support,
            error,
            color,
            light,
        } = self;

        Context {
            pos,
            dist,
            grad,
            uv: t,
            tangent,
            support,
            error,
            color,
            light,
        }
    }
}

impl<P, D, G, U, TA, TB, S, E, C, L> ContextSet<Tangent<TB>>
    for Context<P, D, G, U, TA, S, E, C, L>
{
    type Set = Context<P, D, G, U, Tangent<TB>, S, E, C, L>;

    fn set(self, t: Tangent<TB>) -> Self::Set {
        let Context {
            pos,
            dist,
            grad,
            uv,
            tangent: _,
            support,
            error,
            color,
            light,
        } = self;

        Context {
            pos,
            dist,
            grad,
            uv,
            tangent: t,
            support,
            error,
            color,
            light,
        }
    }
}

impl<P, D, G, U, T, SA, SB, E, C, L> ContextSet<Support<SB>>
    for Context<P, D, G, U, T, SA, E, C, L>
{
    type Set = Context<P, D, G, U, T, Support<SB>, E, C, L>;

    fn set(self, t: Support<SB>) -> Self::Set {
        let Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support: _,
            error,
            color,
            light,
        } = self;

        Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support: t,
            error,
            color,
            light,
        }
    }
}

impl<P, D, G, U, T, S, EA, EB, C, L> ContextSet<BoundingError<EB>>
    for Context<P, D, G, U, T, S, EA, C, L>
{
    type Set = Context<P, D, G, U, T, S, BoundingError<EB>, C, L>;

    fn set(self, t: BoundingError<EB>) -> Self::Set {
        let Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support,
            error: _,
            color,
            light,
        } = self;

        Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support,
            error: t,
            color,
            light,
        }
    }
}

impl<P, D, G, U, T, S, E, CA, CB, L> ContextSet<Color<CB>> for Context<P, D, G, U, T, S, E, CA, L> {
    type Set = Context<P, D, G, U, T, S, E, Color<CB>, L>;

    fn set(self, t: Color<CB>) -> Self::Set {
        let Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support,
            error,
            color: _,
            light,
        } = self;

        Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support,
            error,
            color: t,
            light,
        }
    }
}

impl<P, D, G, U, T, S, E, C, LA, LB> ContextSet<Light<LB>> for Context<P, D, G, U, T, S, E, C, LA> {
    type Set = Context<P, D, G, U, T, S, E, C, Light<LB>>;

    fn set(self, t: Light<LB>) -> Self::Set {
        let Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support,
            error,
            color,
            light: _,
        } = self;

        Context {
            pos,
            dist,
            grad,
            uv,
            tangent,
            support,
            error,
            color,
            light: t,
        }
    }
}
