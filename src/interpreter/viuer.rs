use image::DynamicImage;
use type_fields::{
    macros::Closure,
    t_funk::{closure::Compose, Composed, Curry2, Curry2A, Function, IntoF},
};
use viuer::{Config, ViuResult};

use crate::{Image, Rasterize, ToRgba8};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct ViuerPrint;

impl Function<(Config, DynamicImage)> for ViuerPrint {
    type Output = ViuResult<(u32, u32)>;

    fn call((config, image): (Config, DynamicImage)) -> Self::Output {
        viuer::print(&image, &config)
    }
}

pub type Viuer<D, O, F> = Composed<
    Curry2A<ViuerPrint, Config>,
    Composed<
        IntoF<DynamicImage>,
        Composed<ToRgba8, Composed<IntoF<DynamicImage>, Composed<Image<O, F>, Rasterize<D>>>>,
    >,
>;

pub fn make_viuer_raw<D, O, F>(width: usize, height: usize, config: Config) -> Viuer<D, O, F> {
    Rasterize::<D> {
        width,
        height,
        ..Default::default()
    }
    .compose_l(Image::<O, F>::default())
    .compose_l(IntoF::<DynamicImage>::default())
    .compose_l(ToRgba8) // Convert to RGBA8 for compatibility
    .compose_l(IntoF::<DynamicImage>::default())
    .compose_l(ViuerPrint.prefix2(config))
}

pub fn make_viuer<D, O, F>(w: usize, h: usize) -> Viuer<D, O, F> {
    make_viuer_raw::<D, O, F>(
        w,
        h,
        Config {
            absolute_offset: false,
            width: Some(w as u32),
            ..Default::default()
        },
    )
}
