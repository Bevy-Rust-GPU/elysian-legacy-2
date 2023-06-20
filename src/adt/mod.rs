//! Elysian ADT
//! Elysian = Input a
//! | Field b
//! | Output c
//! | Modify d
//! | Sequence [In|Field|Out|Modify]
//! | Combine Field|Shape|Combine Field|Shape|Combine f
//! where
//!   a: InputModifer
//!   b: FieldMorphism
//!   c: OutputModifier
//!   f: CombineFunction
//!
//! Example:
//!
//! Shape [
//!   In Translate -0.1 -0.3,
//!   Combine (
//!     Shape [
//!       In Translate 0.2 0.2,
//!       Field Point,
//!       Out Isosurface 0.3,
//!     ],
//!     Shape [
//!       In Translate -0.2 -0.2,
//!       Field Point,
//!       Out Isosurface 0.5,
//!       Out Manifold,
//!     ],
//!     Boolean(Lt),
//!   ),
//!   Out Isosurface 0.2,
//! ]
//!

mod impls;

pub use impls::*;

mod bounds;
pub(crate) use bounds::*;

use t_funk::macros::{define_adt, Copointed, Pointed};

define_adt!(
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct ADT
      // Run a computation
      = Run<A>(pub A)
      // Sequence two computations
      | Then<A, B>(pub A, pub B)
      // Combine two computations
      | Combine<A, B, F>(pub A, pub B, pub F)
      // Terminating type
      | AdtEnd;
);

pub use t_funk::op_chain::Done;

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use glam::Vec2;
    use image::{ImageBuffer, Rgb};
    use t_funk::{
        closure::{Closure, ComposeLF, Curry2},
        function::{Id, Lt},
        typeclass::{
            foldable::{FoldMap, Foldl, Foldr},
            functor::{Fmap, FmapF},
            monad::Identity,
        },
    };

    use crate::{
        adt, proxy, BooleanConditional, Combine, Context, ContextA, ContextB, ContextOut,
        ContextRasterImage, CopyContext, CopyProperty, DistGrad, DistGradToRgb, Distance, Done,
        Evaluate, EvaluateSide, Gradient, Inherited, Isosurface, Left, LiftAdtF, LiftEvaluateF,
        LiftParamF, Point, PosDistGrad, Position, Raster, RasterToImage, Rasterizer, Right, Set,
        Translate, UnionS, ViuerPrinter,
    };

    #[test]
    fn test_adt() {
        let shape_a =
            adt() << Translate(Vec2::new(-0.2, -0.2)) << Point << Isosurface(0.8_f32) >> Done;
        let shape_b =
            adt() << Translate(Vec2::new(0.2, 0.2)) << Point << Isosurface(0.8_f32) >> Done;
        let shape_c =
            adt() << Translate(Vec2::new(0.0, 0.4)) << Point << Isosurface(0.8_f32) >> Done;
        let shape_d =
            adt() << Translate(Vec2::new(0.0, -0.4)) << Point << Isosurface(0.8_f32) >> Done;

        let combined = proxy::<Gradient<Vec2>>() << shape_a << shape_b >> Done; // << shape_c >> intersection() << shape_d >> Done;

        let combined = Combine(
            shape_a,
            shape_b,
            (
                EvaluateSide::<Left, Inherited, ContextA>::default(),
                EvaluateSide::<Right, Inherited, ContextB>::default(),
                CopyContext::<ContextA, ContextOut>::default(),
                BooleanConditional(
                    Lt,
                    Id,
                    CopyProperty::<Gradient<Vec2>, ContextB, ContextOut>::default(),
                    PhantomData::<Distance<f32>>,
                ),
            ),
        );

        let _positioned = adt() << Set(Position(Vec2::default())) << combined >> Done;

        /*
        let input = PosDistColor::<(), (), Color<Vec3>>::default();
        let foo = positioned.lift_param(input.clone());
        let foo = LiftCombine::<Dist<f32>>::lift_combine(foo);
        let foo = LiftEvaluate::<Dist<f32>>::lift_evaluate(foo);
        let _foo = foo.call(input);
        let _foo =
            Evaluate::<Dist<f32>, PosDistColor<(), (), Color<Vec3>>>::evaluate(positioned, input);
        */

        pub type ShapeCtxFrom = PosDistGrad<Position<Vec2>, (), ()>;
        pub type ShapeCtxTo = PosDistGrad<(), Distance<f32>, Gradient<Vec2>>;

        //pub type RasterCtx = ContextRasterString<ShapeCtx, ShapeCtx>;
        pub type RasterCtx = ContextRasterImage<
            Context<ShapeCtxFrom>,
            Raster<ShapeCtxFrom>,
            ImageBuffer<Rgb<f32>, Vec<f32>>,
        >;

        let context = RasterCtx::default();

        let rasterizer = adt()
            << Rasterizer::<_, ShapeCtxFrom> {
                width: 48,
                height: 48,
                shape: combined,
                ..Default::default()
            }
            << RasterToImage::<ShapeCtxTo, DistGradToRgb>::default()
            << ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default()
        /*
            << RasterToAscii(ASCII_RAMP, PhantomData::<PosDistGrad<Vec2, f32, Vec2>>)
            << Print
        */
            >> Done;

        Evaluate::<DistGrad<f32, Vec2>, RasterCtx>::evaluate(rasterizer, context);
    }

    #[test]
    fn test_composition() {
        let shape_a = (Translate(Vec2::new(-0.2, -0.2)), Point, Isosurface(0.8_f32));
        let shape_b = (Translate(Vec2::new(0.2, 0.2)), Point, Isosurface(0.8_f32));
        let combined = Combine(shape_a, shape_b, Identity(UnionS));

        let context = PosDistGrad::<Position<Vec2>, Distance<f32>, Gradient<Vec2>>::default();

        let foo = combined;
        let foo = foo.fmap(LiftParamF.suffix2(context));
        let foo = foo.fmap(LiftAdtF);
        let foo = foo.fmap(LiftEvaluateF::<DistGrad<f32, Vec2>>::default());
        let foo = foo.foldr(ComposeLF, Id);
        let foo = foo.call(context);
        panic!("{foo:#?}");
    }
}
