use glam::Vec2;
use std::marker::PhantomData;
use t_funk::{
    closure::Closure,
    function::Lt,
    macros::{functions, types},
    typeclass::{monad::Identity, functor::Fmap},
};

use crate::{
    BlendProperty, BlendPropertyDist, BooleanConditional, Combine, ContextA, ContextB, ContextOut,
    CopyContext, Distance, EvaluateSide, ExpandAlias, Gradient, Inherited, IntoMonad, IntoMonadT,
    Left, Right, LiftAdt, Alias,
};

#[functions]
#[types]
pub trait SmoothUnion<T> {
    type SmoothUnion;

    fn smooth_union(self, rhs: T, k: f32) -> Self::SmoothUnion;
}

impl<T, U> SmoothUnion<U> for T
where
    T: IntoMonad,
    U: IntoMonad,
{
    type SmoothUnion = Combine<IntoMonadT<T>, IntoMonadT<U>, IntoMonadT<SmoothUnionS>>;

    fn smooth_union(self, rhs: U, k: f32) -> Self::SmoothUnion {
        Combine(
            self.into_monad(),
            rhs.into_monad(),
            SmoothUnionS(k).into_monad(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmoothUnionS(f32);

impl<F> Fmap<F> for SmoothUnionS {
    type Fmap = Self;

    fn fmap(self, f: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for SmoothUnionS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for SmoothUnionS {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for SmoothUnionS {
    type ExpandAlias = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        BooleanConditional<
            Lt,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
        BlendProperty<PolynomialSmoothMin<Distance<f32>>, Distance<f32>>,
        BlendPropertyDist<PolynomialSmoothMin<Gradient<Vec2>>, Gradient<Vec2>>,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        (
            EvaluateSide::<Left, Inherited, ContextA>::default(),
            EvaluateSide::<Right, Inherited, ContextB>::default(),
            BooleanConditional(
                Lt,
                CopyContext::default(),
                CopyContext::default(),
                PhantomData::<Distance<f32>>,
            ),
            BlendProperty(
                PolynomialSmoothMin(self.0, PhantomData::<Distance<f32>>),
                PhantomData::<Distance<f32>>,
            ),
            BlendPropertyDist(
                PolynomialSmoothMin(self.0, PhantomData::<Gradient<Vec2>>),
                PhantomData::<Gradient<Vec2>>,
            ),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct PolynomialSmoothMin<T>(pub f32, PhantomData<T>);

impl Closure<(Distance<f32>, Distance<f32>)> for PolynomialSmoothMin<Distance<f32>> {
    type Output = Distance<f32>;

    fn call(self, (Distance(da), Distance(db)): (Distance<f32>, Distance<f32>)) -> Self::Output {
        let t = (0.5 + 0.5 * (db - da) / self.0).clamp(0.0, 1.0);
        let d = db.lerp(da, t) - self.0 * t * (1.0 - t);
        Distance(d)
    }
}

impl<T> Closure<(Distance<f32>, Distance<f32>, T, T)> for PolynomialSmoothMin<T>
where
    T: Lerp<T, f32>,
{
    type Output = LerpT<T, T, f32>;

    fn call(
        self,
        (Distance(da), Distance(db), pa, pb): (Distance<f32>, Distance<f32>, T, T),
    ) -> Self::Output {
        let t = (0.5 + 0.5 * (db - da) / self.0).clamp(0.0, 1.0);
        let p = pb.lerp(pa, t);
        p
    }
}

#[types]
pub trait Lerp<B, T> {
    type Lerp;

    fn lerp(self, b: B, t: T) -> Self::Lerp;
}

impl<A1, B1, A2, B2, T> Lerp<(A2, B2), T> for (A1, B1)
where
    A1: Lerp<A2, T>,
    B1: Lerp<B2, T>,
    T: Clone,
{
    type Lerp = (LerpT<A1, A2, T>, LerpT<B1, B2, T>);

    fn lerp(self, (a2, b2): (A2, B2), t: T) -> Self::Lerp {
        let (a1, b1) = self;
        (a1.lerp(a2, t.clone()), b1.lerp(b2, t))
    }
}

impl<A1, B1, C1, A2, B2, C2, T> Lerp<(A2, B2, C2), T> for (A1, B1, C1)
where
    A1: Lerp<A2, T>,
    B1: Lerp<B2, T>,
    C1: Lerp<C2, T>,
    T: Clone,
{
    type Lerp = (LerpT<A1, A2, T>, LerpT<B1, B2, T>, LerpT<C1, C2, T>);

    fn lerp(self, (a2, b2, c2): (A2, B2, C2), t: T) -> Self::Lerp {
        let (a1, b1, c1) = self;
        (
            a1.lerp(a2, t.clone()),
            b1.lerp(b2, t.clone()),
            c1.lerp(c2, t),
        )
    }
}

impl Lerp<f32, f32> for f32 {
    type Lerp = Self;

    fn lerp(self, b: f32, t: f32) -> Self::Lerp {
        self + (b - self) * t
    }
}

impl Lerp<Distance<f32>, f32> for Distance<f32> {
    type Lerp = Self;

    fn lerp(self, b: Distance<f32>, t: f32) -> Self::Lerp {
        Distance(self.0.lerp(b.0, t))
    }
}

impl Lerp<Gradient<Vec2>, f32> for Gradient<Vec2> {
    type Lerp = Self;

    fn lerp(self, b: Gradient<Vec2>, t: f32) -> Self::Lerp {
        Gradient(self.0.lerp(b.0, t))
    }
}
