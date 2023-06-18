use t_funk::{
    collection::hlist::{Cons, Nil},
    function::Gt,
    typeclass::monad::Identity,
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance, EvaluateSide,
    Inherited, Left, LiftEvaluate, Pair, Right,
};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Then};

#[functions]
#[types]
pub trait Intersection<R> {
    type Intersection;

    fn intersection(self, rhs: R) -> Self::Intersection;
}

pub fn intersection() -> OpChain<LiftAdtF, IntersectionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Intersection<R> for Then<A, B> | Combine<A, B, C> {
        type Intersection = Combine<Self, R, Identity<IntersectionS>>;

        fn intersection(self, rhs: R) -> Self::Intersection {
            Combine(self, rhs, Identity(IntersectionS))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntersectionS;

impl LiftEvaluate<Dist<f32>> for IntersectionS {
    type LiftEvaluate = Cons<
        EvaluateSide<Left, Inherited, ContextA>,
        Cons<
            EvaluateSide<Right, Inherited, ContextB>,
            Cons<
                BooleanConditional<
                    Gt,
                    CopyContext<ContextA, ContextOut>,
                    CopyContext<ContextB, ContextOut>,
                    Distance<f32>,
                >,
                Nil,
            >,
        >,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}

impl<D> LiftEvaluate<(Distance<f32>, D)> for IntersectionS
where
    D: Pair,
{
    type LiftEvaluate = Cons<
        EvaluateSide<Left, Dist<f32>, ContextA>,
        Cons<
            EvaluateSide<Right, Dist<f32>, ContextB>,
            Cons<
                BooleanConditional<
                    Gt,
                    EvaluateSide<Left, Inherited, ContextOut>,
                    EvaluateSide<Right, Inherited, ContextOut>,
                    Distance<f32>,
                >,
                Nil,
            >,
        >,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}
