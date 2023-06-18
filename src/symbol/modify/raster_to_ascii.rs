use std::marker::PhantomData;

use t_funk::{
    closure::{Curry2, Curry2A},
    collection::set::{Get, GetF},
    function::Mul,
    macros::lift,
    typeclass::{
        copointed::{CopointF, Copointed},
        functor::Fmap,
    },
};

use crate::{
    AsUsize, CharsToString, Distance, Evaluable, EvaluateFunction, Index, Invert, LiftAdt,
    LiftModify, Raster, Run, Saturate,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RasterToAscii<const N: usize, R>(pub Ramp<N>, pub PhantomData<R>);

impl<const N: usize, R, F> Fmap<F> for RasterToAscii<N, R> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<const N: usize, R> LiftAdt for RasterToAscii<N, R> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<const N: usize, R> Evaluable for RasterToAscii<N, R> {
    type Evaluable = LiftModify;
}

impl<const N: usize, R, D> EvaluateFunction<D> for RasterToAscii<N, R> {
    type Inputs = Raster<R>;
    type Moves = Raster<R>;
    type Function = Curry2A<Ascii, Ramp<N>>;

    fn evaluate_function(self) -> Self::Function {
        Ascii.prefix2(self.0)
    }
}

pub type Ramp<const N: usize> = [char; N];
pub const ASCII_RAMP: Ramp<11> = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '█'];

#[lift]
pub fn ascii<const N: usize, R>(ramp: Ramp<N>, rast: Raster<R>) -> String
where
    R: Clone + Get<Distance<f32>>,
{
    rast.fmap(GetF::<Distance<f32>>::default())
        .fmap(CopointF)
        .fmap(Saturate)
        .fmap(Invert)
        .fmap(Mul.suffix2((N - 1) as f32))
        .fmap(AsUsize)
        .fmap(Index.prefix2(ramp))
        .copoint()
        .fmap(|mut line: Vec<char>| {
            line.push('\n');
            line
        })
        .fmap(CharsToString)
        .into_iter()
        .collect()
}
