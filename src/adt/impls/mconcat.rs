use t_funk::{
    macros::impl_adt,
    typeclass::{
        foldable::{Foldl, FoldlT},
        monoid::{Mconcat, Mempty, MemptyT},
        semigroup::MappendF,
    },
};

use crate::{AdtEnd, Combine, Run, Then};

impl Mconcat for AdtEnd {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}

impl_adt! {
    impl<A, B> Mconcat for Run<A> | Then<A, B>
    where
        A: Mempty,
        Self: Foldl<MappendF, MemptyT<A>>,
    {
        type Mconcat = FoldlT<Self, MappendF, MemptyT<A>>;

        fn mconcat(self) -> Self::Mconcat {
            self.foldl(MappendF::default(), A::mempty())
        }
    }
}

impl<A, B, C> Mconcat for Combine<A, B, C>
where
    A: Mempty,
    B: Mempty<Mempty = MemptyT<A>>,
    Self: Foldl<MappendF, MemptyT<A>>,
{
    type Mconcat = FoldlT<Self, MappendF, MemptyT<A>>;

    fn mconcat(self) -> Self::Mconcat {
        self.foldl(MappendF::default(), A::mempty())
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        closure::Const,
        op_chain::Done,
        typeclass::{functor::Fmap, monoid::Mconcat, semigroup::Sum},
    };

    use crate::{adt, Isosurface, Point, Translate};

    #[test]
    fn test_adt_mconcat() {
        let adt = adt() << Translate(Vec2::new(0.0, 0.0)) << Point << Isosurface(0.0) >> Done;
        let foo = adt.fmap(Const((Sum(1),)));
        assert_eq!(foo.mconcat(), (Sum(1), Sum(1), Sum(1)));
    }
}
