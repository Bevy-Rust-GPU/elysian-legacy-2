extern crate alloc;

use alloc::string::String;

use crate::{
    AsUsize, CharsToString, Distance, DistanceF32, Index, Invert, PosDistGrad, Raster, Saturate,
};

use type_fields::{
    macros::Closure,
    t_funk::{
        closure::Compose,
        set::{Get, GetF},
        Composed, CopointF, Copointed, Curry2, Curry2A, Fmap, Function, Mul, PrintLn,
    },
};

use super::Rasterize;

pub type Ramp<const N: usize> = [char; N];
pub const ASCII_RAMP: Ramp<11> = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '█'];

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Ascii;

impl<const N: usize, C> Function<(Ramp<N>, Raster<C>)> for Ascii
where
    C: Clone + Get<DistanceF32>,
{
    type Output = String;

    fn call((ramp, rast): (Ramp<N>, Raster<C>)) -> Self::Output {
        rast.fmap(GetF::<DistanceF32>::default())
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
}

pub fn make_ascii(
    width: usize,
    height: usize,
) -> Composed<
    PrintLn,
    Composed<Curry2A<Ascii, [char; 11]>, Rasterize<(Distance<f32>, ()), PosDistGrad>>,
> {
    Rasterize {
        width,
        height,
        ..Default::default()
    }
    .compose_l(Ascii.prefix2(ASCII_RAMP))
    .compose_l(PrintLn)
}

