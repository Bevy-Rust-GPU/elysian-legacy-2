#![no_std]
#![feature(return_position_impl_trait_in_trait)]

pub mod combine;
pub mod compose;
pub mod context;
pub mod field_traits;
pub mod fields;
pub mod fn_traits;
pub mod lifting;
pub mod modifiers;
pub mod util;

use crate::{
    combine::*, compose::*, context::*, fields::infinity::Infinity,
    modifiers::isosurface::Isosurface,
};
use combine::{
    blend::{blend, blend_union_color, copy_context_lhs, mod_color_by_distance, uv_to_position},
    evaluate::{evaluate_left, evaluate_right},
};
use field_traits::Field;
use fields::{chebyshev::Chebyshev, quad::Quad, ring::Ring, triangle::Triangle};
use fn_traits::Func1;
use lifting::lift_blend_evaluate;
use modifiers::{
    ambient_light::AmbientLight, aspect_adaptive_plus::AspectAdaptivePlus,
    bounding_error_to_color::BoundingErrorToColor, derive_bounding_error::DeriveBoundingError,
    derive_support_function::DeriveSupportFunction, derive_tangent::DeriveTangent,
    directional_light::DirectionalLight, distance_to_color::DistanceToColor,
    gradient_to_color::GradientToColor, light_to_color::LightToColor, lighting::Lighting,
    offset_uv::OffsetUv, rotate::Rotate, scale::Scale, scale_uv::ScaleUv,
    support_to_color::SupportToColor, tangent_to_color::TangentToColor, translate::Translate,
    twist::Twist, uv_to_color::UvToColor, wrap_uv::WrapUv,
};
use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Cos, Sin,
};
use util::{constant, frag_width};

pub fn test_program(p: Vec2, viewport: Vec2, time: f32) -> Color<Vec3> {
    let k_frag = frag_width(0.5);
    let k_blend = constant(0.25);

    let uvs = compose!(
        ScaleUv::field(Vec2::splat(4.0)),
        WrapUv::field(Vec2::splat(1.0)),
        ScaleUv::field(Vec2::splat(2.0)),
        OffsetUv::field(Vec2::splat(-1.0))
    );

    // Define shapes
    let background_black = move |i| (Infinity::field(()), SetColor::field(Vec3::ZERO)).compose(i);

    let ring_red = Rotate::field(-time, move |i| {
        (
            Ring::field((0.45 - time.cos() * 0.75, 0.125, k_frag)),
            uvs,
            SetColor::field(Vec3::X),
        )
            .compose(i)
    });

    let ring_green = Rotate::field(time, move |i| {
        (
            Ring::field((0.55 + time.sin() * 0.5, 0.125, k_frag)),
            uvs,
            SetColor::field(Vec3::Y),
        )
            .compose(i)
    });

    let triangle_blue = Rotate::field(
        time,
        Twist::field(-core::f32::consts::PI, move |i| {
            (
                Triangle::field((0.8, k_frag)),
                uvs,
                SetColor::field(Vec3::Z),
            )
                .compose(i)
        }),
    );

    // Compose shapes
    let shape = Scale::field(
        (time * 0.1).sin() * 0.25 + 0.5,
        blend_background(
            background_black,
            k_frag,
            blend_over(
                ring_red,
                k_blend,
                blend_over(ring_green, k_blend, triangle_blue),
            ),
        ),
    );

    let quad = move |i| (Quad::field(Vec2::splat(0.2)), Isosurface::field(0.04)).compose(i);

    // UV map main field onto quads
    let panel_dist = uv_map(
        move |i| (Translate::field(Vec2::new(-1.0, 0.5)), quad).compose(i),
        move |i| (shape, DistanceToColor::field()).compose(i),
    );

    let panel_grad = uv_map(
        compose!(Translate::field(Vec2::new(0.0, 0.5)), quad),
        compose!(shape, GradientToColor::field(k_frag)),
    );

    let panel_tangent = uv_map(
        compose!(Translate::field(Vec2::new(0.0, 0.0)), quad),
        compose!(shape, DeriveTangent::field(), TangentToColor::field(k_frag)),
    );

    let support_derived = compose!(shape, DeriveSupportFunction::field());

    let panel_support = uv_map(
        compose!(Translate::field(Vec2::new(-0.5, 0.0)), quad),
        compose!(support_derived, SupportToColor::field()),
    );

    let panel_error = uv_map(
        compose!(Translate::field(Vec2::new(-0.5, -0.5)), quad),
        compose!(
            DeriveBoundingError::field(support_derived),
            BoundingErrorToColor::field(),
        ),
    );

    let panel_uv = uv_map(
        compose!(Translate::field(Vec2::new(1.0, 0.5)), quad),
        compose!(shape, UvToColor::field()),
    );

    let uv_mapped = combine(
        shape,
        compose!(
            evaluate_left,
            lift_blend_evaluate(copy_context_lhs),
            lift_blend_evaluate(uv_to_position),
            evaluate_right,
            blend(mod_color_by_distance)
        ),
        Chebyshev::field(()),
    );

    let panel_color = uv_map(
        compose!(Translate::field(Vec2::new(1.0, 0.0)), quad),
        uv_mapped,
    );

    let panel_light = uv_map(
        compose!(Translate::field(Vec2::new(0.5, 0.0)), quad),
        compose!(
            shape,
            Set::field(Light(0.0)),
            AmbientLight::field(0.2),
            DirectionalLight::field(Vec3::ONE.normalize(), 1.0),
            LightToColor::field()
        ),
    );

    let panel_lighting = uv_map(
        compose!(Translate::field(Vec2::new(0.75, -0.5)), quad),
        compose!(
            uv_mapped,
            Set::field(Light(0.0)),
            AmbientLight::field(0.2),
            DirectionalLight::field(Vec3::ONE.normalize(), 1.0),
            Lighting::field()
        ),
    );

    let fin = blend_background(
        background_black,
        k_frag,
        fold!(blend_union_color => (
            panel_dist,
            panel_grad,
            panel_tangent,
            panel_support,
            panel_error,
            panel_uv,
            panel_color,
            panel_light,
            panel_lighting
        )),
    );

    let fin = compose!(
        Set::field(Position(p)),
        AspectAdaptivePlus::field(viewport),
        fin
    );

    // Evaluate
    let ctx = fin(Context::default());

    ctx.get()
}

#[cfg(test)]
mod test {
    use rust_gpu_bridge::glam::Vec3;

    use crate::context::Color;

    #[test]
    fn test_program() {
        let res = super::test_program(Default::default(), Default::default(), 0.0);

        assert_eq!(res, Color(Vec3::ZERO))
    }
}
