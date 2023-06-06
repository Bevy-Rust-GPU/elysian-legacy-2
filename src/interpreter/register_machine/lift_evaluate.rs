use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, Compose, ComposeLT},
    macros::{functions, impl_adt, types},
};

use crate::{
    Combine, Field, Input, LiftDomains, LiftDomainsT, Modify, Nil, NotNil, Output, Sequence, Unit,
};

#[functions]
#[types]
pub trait LiftEvaluate<D> {
    type LiftEvaluate;

    fn lift_evaluate(self) -> Self::LiftEvaluate;
}

impl_adt! {
    impl<A, B, D> LiftEvaluate<D> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        D: LiftDomains<A>,
        B: NotNil + LiftEvaluate<D>,
    {
        type LiftEvaluate = ComposeLT<LiftDomainsT<D, A>, LiftEvaluateT<B, D>>;

        fn lift_evaluate(self) -> Self::LiftEvaluate {
            D::lift_domains(self.0).compose_l(self.1.lift_evaluate())
        }
    }
}

impl_adt! {
    impl<A, D> LiftEvaluate<D> for Input<A, Nil> | Field<A, Nil> | Output<A,Nil>
    where
        D: LiftDomains<A>,
    {
        type LiftEvaluate = LiftDomainsT<D, A>;

        fn lift_evaluate(self) -> Self::LiftEvaluate {
            D::lift_domains(self.0)
        }
    }
}

impl<T, D> LiftEvaluate<D> for Modify<T>
where
    T: LiftEvaluate<D>,
{
    type LiftEvaluate = LiftEvaluateT<T, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate()
    }
}

impl<T, D> LiftEvaluate<D> for Unit<T>
where
    T: LiftEvaluate<D>,
{
    type LiftEvaluate = LiftEvaluateT<T, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate()
    }
}

impl<A, B, D> LiftEvaluate<D> for Sequence<A, B>
where
    A: LiftEvaluate<D>,
    B: LiftEvaluate<D>,
    LiftEvaluateT<A, D>: Compose<LiftEvaluateT<B, D>>,
{
    type LiftEvaluate = ComposeLT<LiftEvaluateT<A, D>, LiftEvaluateT<B, D>>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate().compose_l(self.1.lift_evaluate())
    }
}

impl<A, B, F, D> LiftEvaluate<D> for Combine<A, B, F> {
    type LiftEvaluate = LiftEvaluateCombine<A, B, F, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        LiftEvaluateCombine(self.0, self.1, self.2, PhantomData)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftEvaluateCombine<A, B, F, D>(A, B, F, PhantomData<D>);

impl<A, B, F, D, C> Closure<C> for LiftEvaluateCombine<A, B, F, D>
where
    F: Closure<(A, B, C, PhantomData<D>), Output = C>,
{
    type Output = C;

    fn call(self, input: C) -> Self::Output {
        self.2.call((self.0, self.1, input, PhantomData))
    }
}
