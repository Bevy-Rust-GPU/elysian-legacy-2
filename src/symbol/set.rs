use t_funk::{
    closure::{Closure, OutputT},
    closure::{Curry2, Curry2B},
    collection::set::InsertF,
    typeclass::functor::Fmap,
};

use crate::{LiftAdt, LiftEvaluate, LiftParam, LiftParamT, Run};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Set<T>(pub T);

impl<T, F> Fmap<F> for Set<T>
where
    F: Closure<T>,
{
    type Fmap = Set<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Set(f.call(self.0))
    }
}

impl<T> LiftAdt for Set<T> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, D> LiftEvaluate<D> for Set<T> {
    type LiftEvaluate = Curry2B<InsertF, T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        InsertF.suffix2(self.0)
    }
}

impl<T, C> LiftParam<C> for Set<T>
where
    T: LiftParam<C>,
{
    type LiftParam = Set<LiftParamT<T, C>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Set(self.0.lift_param(input))
    }
}
