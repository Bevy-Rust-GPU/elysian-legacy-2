use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldl, FoldlT},
};

use crate::{AdtEnd, Combine, Run, Then};

impl<F, Z> Foldl<F, Z> for AdtEnd {
    type Foldl = Z;

    fn foldl(self, _: F, z: Z) -> Self::Foldl {
        z
    }
}

impl<A, F, Z> Foldl<F, Z> for Run<A>
where
    F: Closure<(Z, A)>,
{
    type Foldl = OutputT<F, (Z, A)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl_adt! {
    impl<A, B, C, F, Z> Foldl<F, Z> for Then<A, B> | Combine<A, B, C>
    where
        A: Foldl<F, Z>,
        B: Foldl<F, FoldlT<A, F, Z>>,
        F: Clone,
    {
        type Foldl = FoldlT<B, F, FoldlT<A, F, Z>>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            self.1.foldl(f.clone(), self.0.foldl(f, z))
        }
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        function::FormatDebug, macros::lift, op_chain::Done, typeclass::foldable::Foldl,
        typeclass::functor::Fmap,
    };

    use crate::{adt, Isosurface, Point, Translate};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldl() {
        let adt = adt() << Translate(Vec2::new(0.0, 0.0)) << Point << Isosurface(0.0) >> adt() >> Done;
        let folded = adt.fmap(FormatDebug).foldl(Concat, String::default());

        assert_eq!(folded, "Input(Translate(Vec2(0.0, 0.0)), Field(Point, Output(Isosurface(0.0), ShapeEnd)))")
    }
}
