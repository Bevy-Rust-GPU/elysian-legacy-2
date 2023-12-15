pub mod blend;
pub mod evaluate;

use crate::{compose, context::*, fn_traits::Func1, lifting::lift_blend_evaluate};

use core::marker::PhantomData;
use rust_gpu_bridge::glam::{Vec2, Vec3};

use self::{
    blend::{
        blend, copy_context_rhs, copy_property_lhs, smooth_overlay_prop, smooth_union,
        smooth_union_prop, union, uv_to_position,
    },
    evaluate::{evaluate_both, evaluate_left, evaluate_right},
};

pub fn combine<CI, FL, FR, CO>(
    fa: FL,
    cf: impl Func1<(FL, FR, CI, CI, Context), CO>,
    fb: FR,
) -> impl Func1<CI, CO>
where
    CI: Clone,
    FL: Copy,
    FR: Copy,
{
    move |c: CI| cf((fa, fb, c.clone(), c, Context::default()))
}

pub fn uv_map<CI, LB, RB, CO>(a: impl Func1<CI, LB>, b: impl Func1<CI, RB>) -> impl Func1<CI, CO>
where
    CI: Clone + ContextGet<Position<Vec2>> + ContextSet<Position<Vec2>, Set = CI>,
    LB: ContextGet<Distance<f32>> + ContextGet<Uv<Vec2>> + ContextSet<Position<Vec2>, Set = LB>,
    RB: Clone + ContextGet<Distance<f32>> + ContextSet<Distance<f32>, Set = CO>,
{
    combine(
        a,
        compose!(
            evaluate_left,
            lift_blend_evaluate(uv_to_position),
            evaluate_right,
            blend(compose!(
                copy_context_rhs,
                copy_property_lhs(PhantomData::<Distance<f32>>),
            )),
        ),
        b,
    )
}

pub fn blend_background<CI, LB, RB, CO>(
    a: impl Func1<CI, LB>,
    k: impl Func1<f32, f32>,
    b: impl Func1<CI, RB>,
) -> impl Func1<CI, CO>
where
    CI: Clone,
    LB: ContextGet<Distance<f32>> + ContextGet<Color<Vec3>> + ContextSet<Color<Vec3>>,
    RB: Clone
        + ContextGet<Distance<f32>>
        + ContextGet<Color<Vec3>>
        + ContextSet<Color<Vec3>, Set = CO>,
{
    combine(
        a,
        compose!(
            evaluate_both,
            blend(compose!(
                copy_context_rhs,
                smooth_overlay_prop(PhantomData::<Color<Vec3>>, k),
            ))
        ),
        b,
    )
}

pub fn blend_over<CI, CO>(
    a: impl Func1<CI, CO>,
    k: impl Func1<f32, f32>,
    b: impl Func1<CI, CO>,
) -> impl Func1<CI, CO>
where
    CI: Clone,
    CO: Clone
        + ContextGet<Distance<f32>>
        + ContextSet<Distance<f32>, Set = CO>
        + ContextGet<Gradient<Vec2>>
        + ContextSet<Gradient<Vec2>, Set = CO>
        + ContextGet<Uv<Vec2>>
        + ContextSet<Uv<Vec2>, Set = CO>
        + ContextGet<Color<Vec3>>
        + ContextSet<Color<Vec3>, Set = CO>,
{
    combine(
        a,
        compose!(
            evaluate_both,
            blend(compose!(
                union,
                smooth_union(k),
                smooth_union_prop(PhantomData::<Gradient<Vec2>>, k),
                smooth_union_prop(PhantomData::<Uv<Vec2>>, k),
                smooth_union_prop(PhantomData::<Color<Vec3>>, k),
            ))
        ),
        b,
    )
}
