use std::marker::PhantomData;

use crate::{
    interpreter::register_machine::evaluate_function::MovesT, EvaluateFunction, EvaluateInputs,
    LiftEvaluateFunctionF, Pair,
};

use t_funk::{
    closure::{CallF, Closure, Compose, ComposeLF, ComposeLT, Curry2, Curry2B, OutputT},
    collection::set::{EmptyF, InsertF, SubtractFromF},
    function::{Function, Id},
    macros::{types, Closure},
    typeclass::arrow::{Fanout, FanoutF, FanoutT, Second, SecondT},
};

/// Given a structure of shape subtypes, and a list of domains,
/// produce a function that takes a context, reads input from it,
/// evaluates the relevant domain functions, and produces an updated context
#[types]
pub trait LiftEvaluates<T> {
    type LiftEvaluates;

    fn lift_evaluates(input: T) -> Self::LiftEvaluates;
}

#[derive(Closure)]
pub struct LiftEvaluatesF<T>(PhantomData<T>);

impl<T> Default for LiftEvaluatesF<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for LiftEvaluatesF<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for LiftEvaluatesF<T> {}

impl<D, T> Function<T> for LiftEvaluatesF<D>
where
    D: LiftEvaluates<T>,
{
    type Output = LiftEvaluatesT<D, T>;

    fn call(input: T) -> Self::Output {
        D::lift_evaluates(input)
    }
}

impl<D, N, T> LiftEvaluates<T> for (D, N)
where
    Self: LiftDomainsList<T> + FanoutSetters + ComposeSetters + ComposeRemoves<T>,
    LiftDomainsListT<Self, T>: Closure<T>,
    FanoutSettersT<Self>: Closure<OutputT<LiftDomainsListT<Self, T>, T>>,
{
    type LiftEvaluates = ComposeLT<
        FanoutT<
            ComposeLT<
                OutputT<FanoutSettersT<Self>, OutputT<LiftDomainsListT<Self, T>, T>>,
                ComposeSettersT<Self>,
            >,
            ComposeLT<FanoutT<ComposeLT<EmptyF, ComposeRemovesT<Self, T>>, Id>, SubtractFromF>,
        >,
        CallF,
    >;

    fn lift_evaluates(input: T) -> Self::LiftEvaluates {
        Self::lift_domain_list()
            .compose_l(Self::fanout_setters())
            .call(input)
            .compose_l(Self::compose_setters())
            .fanout(
                EmptyF
                    .compose_l(Self::compose_removes())
                    .fanout(Id)
                    .compose_l(SubtractFromF),
            )
            .compose_l(CallF)
    }
}

/// Given a list of domain types and an ADT implementing those domains,
/// produce a fanout structure of context-lifted domain functions
#[types]
pub trait LiftDomainsList<T> {
    type LiftDomainsList;

    fn lift_domain_list() -> Self::LiftDomainsList;
}

impl<D, N, T> LiftDomainsList<T> for (D, N)
where
    T: EvaluateFunction<D>,
    N: LiftDomainsList<T>,
    LiftEvaluateFunctionF<D>: Fanout<LiftDomainsListT<N, T>>,
    N: Pair,
{
    type LiftDomainsList = FanoutT<LiftEvaluateFunctionF<D>, LiftDomainsListT<N, T>>;

    fn lift_domain_list() -> Self::LiftDomainsList {
        LiftEvaluateFunctionF::<D>::default().fanout(<N as LiftDomainsList<T>>::lift_domain_list())
    }
}

impl<D, T> LiftDomainsList<T> for (D, ())
where
    T: EvaluateFunction<D>,
{
    type LiftDomainsList = LiftEvaluateFunctionF<D>;

    fn lift_domain_list() -> Self::LiftDomainsList {
        LiftEvaluateFunctionF::<D>::default()
    }
}

/// Given a list of domain functions,
/// produce a function that will fanout their resulting setters
#[types]
pub trait FanoutSetters {
    type FanoutSetters;

    fn fanout_setters() -> Self::FanoutSetters;
}

impl<D, N, N2> FanoutSetters for (D, (N, N2))
where
    (N, N2): FanoutSetters,
    N2: Pair,
{
    type FanoutSetters = ComposeLT<SecondT<FanoutF>, FanoutSettersT<(N, N2)>>;

    fn fanout_setters() -> Self::FanoutSetters {
        FanoutF.second().compose_l(<(N, N2)>::fanout_setters())
    }
}

impl<D, N> FanoutSetters for (D, (N, ())) {
    type FanoutSetters = FanoutF;

    fn fanout_setters() -> Self::FanoutSetters {
        FanoutF
    }
}

impl<D> FanoutSetters for (D, ()) {
    type FanoutSetters = Id;

    fn fanout_setters() -> Self::FanoutSetters {
        Id
    }
}

/// Given a list of domain functions,
/// produce a function that will compose the fanout structure
/// of their resulting setters
#[types]
pub trait ComposeSetters {
    type ComposeSetters;

    fn compose_setters() -> Self::ComposeSetters;
}

impl<D, N, N2> ComposeSetters for (D, (N, N2))
where
    (N, N2): ComposeSetters,
    N2: Pair,
{
    type ComposeSetters = ComposeLT<SecondT<ComposeLF>, ComposeSettersT<(N, N2)>>;

    fn compose_setters() -> Self::ComposeSetters {
        ComposeLF.second().compose_l(<(N, N2)>::compose_setters())
    }
}

impl<D, N> ComposeSetters for (D, (N, ())) {
    type ComposeSetters = ComposeLF;

    fn compose_setters() -> Self::ComposeSetters {
        ComposeLF
    }
}

impl<D> ComposeSetters for (D, ()) {
    type ComposeSetters = Id;

    fn compose_setters() -> Self::ComposeSetters {
        Id
    }
}

#[types]
pub trait ComposeRemoves<T> {
    type ComposeRemoves;

    fn compose_removes() -> Self::ComposeRemoves;
}

impl<D, N, T> ComposeRemoves<T> for (D, N)
where
    T: EvaluateInputs<D>,
    MovesT<T, D>: Default,
    N: Pair + ComposeRemoves<T>,
{
    type ComposeRemoves = ComposeLT<Curry2B<InsertF, MovesT<T, D>>, ComposeRemovesT<N, T>>;

    fn compose_removes() -> Self::ComposeRemoves {
        InsertF
            .suffix2(MovesT::<T, D>::default())
            .compose_l(N::compose_removes())
    }
}

impl<D, T> ComposeRemoves<T> for (D, ())
where
    T: EvaluateInputs<D>,
    MovesT<T, D>: Default,
{
    type ComposeRemoves = Curry2B<InsertF, MovesT<T, D>>;

    fn compose_removes() -> Self::ComposeRemoves {
        InsertF.suffix2(MovesT::<T, D>::default())
    }
}
