use std::marker::PhantomData;

use glam::{Vec2, Vec3};
use image::{ImageBuffer, Luma, Pixel, Rgb};
use t_funk::{
    closure::{Closure, OutputT},
    collection::set::Get,
    macros::{
        lift,
        phantom::{PhantomClone, PhantomCopy, PhantomDefault},
        Closure,
    },
    typeclass::{copointed::Copointed, functor::Fmap},
};

use crate::{Color, Distance, Gradient, Invert, LiftAdt, Modify, ModifyFunction, Raster, Saturate};

#[derive(
    Debug, PhantomDefault, PhantomCopy, PhantomClone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct RasterToImage<C, F>(pub PhantomData<(C, F)>);

impl<C, G, F> Fmap<F> for RasterToImage<C, G> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<C, F> LiftAdt for RasterToImage<C, F> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<C, F, D> ModifyFunction<D> for RasterToImage<C, F> {
    type Inputs = Raster<C>;

    type Function = Image<C, F>;

    fn modify_function(self) -> Self::Function {
        Image(self.0)
    }
}

#[lift]
pub fn dist_to_luma<C>(c: C) -> Luma<f32>
where
    C: Get<Distance<f32>>,
{
    *Pixel::from_slice(&[c.get().fmap(Saturate).fmap(Invert).copoint()])
}

#[lift]
pub fn dist_grad_to_rgb<C>(c: C) -> Rgb<f32>
where
    C: Get<(Distance<f32>, Gradient<Vec2>)>,
{
    let (Distance(dist), Gradient(g)) = c.get();

    let c = if dist <= 0.0 {
        [g.x * 0.5 + 0.5, g.y * 0.5 + 0.5, 1.0 - dist]
    } else {
        [g.x * 0.5 + 0.5, g.y * 0.5 + 0.5, 0.0]
    };

    *Pixel::from_slice(&c)
}

#[lift]
pub fn dist_color_to_rgb<C>(c: C) -> Rgb<f32>
where
    C: Get<(Distance<f32>, Color<Vec3>)>,
{
    let (Distance(dist), Color(c)) = c.get();

    let l = (-dist).max(0.0).min(1.0);
    let c = [c.x * l, c.y * l, c.z * l];

    *Pixel::from_slice(&c)
}

#[derive(Closure)]
pub struct Image<C, F>(pub PhantomData<(C, F)>);

impl<C, F> std::fmt::Debug for Image<C, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Image").field(&self.0).finish()
    }
}

impl<C, F> Default for Image<C, F> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C, F> Clone for Image<C, F> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<C, F> Copy for Image<C, F> {}

impl<C, F> Closure<Raster<C>> for Image<C, F>
where
    C: Clone,
    F: Default + Closure<C>,
    OutputT<F, C>: Pixel,
{
    type Output = ImageBuffer<OutputT<F, C>, Vec<<OutputT<F, C> as Pixel>::Subpixel>>;

    fn call(self, rast: Raster<C>) -> Self::Output {
        let w = rast[0].len() as u32;
        let h = rast.len() as u32;

        let mut buf = ImageBuffer::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let dist = F::default().call(rast[y as usize][x as usize].clone());
                buf.put_pixel(x, y, dist);
            }
        }

        buf
    }
}
