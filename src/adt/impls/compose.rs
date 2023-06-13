//! Composition of ADT types

use t_funk::{
    macros::impl_adt,
    typeclass::category::{Compose, ComposeT, Id},
};

use crate::{AdtEnd, Combine, NotAdtEnd, Run, Then};

// AdtEnd is the compositional identity

impl_adt! {
    impl<A, B, C> Id for AdtEnd | Run<A> | Then<A, B> | Combine<A, B, C> {
        type Id = AdtEnd;

        fn id() -> Self::Id {
            AdtEnd
        }
    }
}

impl<A> Compose<Run<A>> for AdtEnd {
    type Compose = Run<A>;

    fn compose(self, f: Run<A>) -> Self::Compose {
        f
    }
}

impl<A, B> Compose<Then<A, B>> for AdtEnd
where
    Self: Compose<A>,
{
    type Compose = Then<A, B>;

    fn compose(self, f: Then<A, B>) -> Self::Compose {
        f
    }
}

impl<A, B, C> Compose<Combine<A, B, C>> for AdtEnd {
    type Compose = Combine<A, B, C>;

    fn compose(self, f: Combine<A, B, C>) -> Self::Compose {
        f
    }
}

// Composition with AdtEnd

impl_adt! {
    impl<A, B, C> Compose<AdtEnd> for AdtEnd | Run<A> |  Combine<A, B, C> {
        type Compose = Self;

        fn compose(self, _: AdtEnd) -> Self::Compose {
            self
        }
    }
}

impl<A> Compose<AdtEnd> for Then<A, AdtEnd> {
    type Compose = Self;

    fn compose(self, _: AdtEnd) -> Self::Compose {
        self
    }
}

// Run

impl<A, B> Compose<Run<B>> for Run<A> {
    type Compose = Then<Self, Then<Run<B>, AdtEnd>>;

    fn compose(self, f: Run<B>) -> Self::Compose {
        Then(self, Then(f, AdtEnd))
    }
}

impl<A, B, C> Compose<Then<B, C>> for Run<A>
where
    Self: Compose<B>,
{
    type Compose = Then<Self, Then<B, C>>;

    fn compose(self, f: Then<B, C>) -> Self::Compose {
        Then(self, f)
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Run<A> {
    type Compose = Then<Self, Then<Combine<B, C, F>, AdtEnd>>;

    fn compose(self, f: Combine<B, C, F>) -> Self::Compose {
        Then(self, Then(f, AdtEnd))
    }
}

// Then

impl<A, B, C> Compose<C> for Then<A, B>
where
    B: NotAdtEnd + Compose<C>,
    C: NotAdtEnd,
{
    type Compose = Then<A, B::Compose>;

    fn compose(self, rhs: C) -> Self::Compose {
        Then(self.0, self.1.compose(rhs))
    }
}

impl<A, B> Compose<B> for Then<A, AdtEnd>
where
    A: Compose<B>,
    B: NotAdtEnd,
{
    type Compose = ComposeT<A, B>;

    fn compose(self, rhs: B) -> Self::Compose {
        self.0.compose(rhs)
    }
}

// Combine

impl<A, B, F, C> Compose<Run<C>> for Combine<A, B, F> {
    type Compose = Then<Self, Then<Run<C>, AdtEnd>>;

    fn compose(self, rhs: Run<C>) -> Self::Compose {
        Then(self, Then(rhs, AdtEnd))
    }
}

impl<A, B, F, C, D> Compose<Then<C, D>> for Combine<A, B, F>
where
    Self: Compose<C>,
{
    type Compose = Then<Self, Then<C, D>>;

    fn compose(self, rhs: Then<C, D>) -> Self::Compose {
        Then(self, rhs)
    }
}

impl<A, B, F, C, D, G> Compose<Combine<C, D, G>> for Combine<A, B, F> {
    type Compose = Then<Self, Combine<C, D, G>>;

    fn compose(self, rhs: Combine<C, D, G>) -> Self::Compose {
        Then(self, rhs)
    }
}
