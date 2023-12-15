use crate::{compose, context::*, fn_traits::Func1};

use core::marker::PhantomData;
use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Mix,
};

pub fn blend_begin<FL, FR, CA, CB, CC>((_, _, ca, cb, ci): (FL, FR, CA, CB, CC)) -> (CA, CB, CC) {
    (ca, cb, ci)
}

pub fn blend_end<CA, CB, CI>((_, _, ci): (CA, CB, CI)) -> CI {
    ci
}

pub fn blend<FA, FB, LA, RA, LB, RB, CI, CO>(
    f: impl Func1<(LA, RA, CI), (LB, RB, CO)>,
) -> impl Func1<(FA, FB, LA, RA, CI), CO> {
    move |c| blend_end(f(blend_begin(c)))
}

pub fn copy_context_lhs<CL, CR, CC>((cl, cr, _): (CL, CR, CC)) -> (CL, CR, CL)
where
    CL: Clone,
{
    (cl.clone(), cr, cl)
}

pub fn copy_context_rhs<CL, CR, CC>((cl, cr, _): (CL, CR, CC)) -> (CL, CR, CR)
where
    CR: Clone,
{
    (cl, cr.clone(), cr)
}

pub fn uv_to_position<CL, CR, CO, CC>((cl, cr, cc): (CL, CR, CC)) -> (CL, CO, CC)
where
    CL: ContextGet<Uv<Vec2>>,
    CR: ContextSet<Position<Vec2>, Set = CO>,
{
    let Uv(u) = cl.get();
    (cl, cr.set(Position(u)), cc)
}

pub fn union<C, CC>((ca, cb, _): (C, C, CC)) -> (C, C, C)
where
    C: Clone + ContextGet<Distance<f32>>,
{
    let Distance(da) = ca.get();
    let Distance(db) = cb.get();

    if da < db {
        (ca.clone(), cb, ca)
    } else {
        (ca, cb.clone(), cb)
    }
}

pub fn copy_property_lhs<P, CL, CR, CC, CO>(
    _: PhantomData<P>,
) -> impl Func1<(CL, CR, CC), (CL, CR, CO)>
where
    CL: ContextGet<P>,
    CR: ContextGet<P>,
    CC: ContextSet<P, Set = CO>,
{
    move |(ca, cb, ci): (CL, CR, CC)| {
        let p = ca.get();
        (ca, cb, ci.set(p))
    }
}

pub fn copy_property_rhs<P, CL, CR, CC, CO>(
    _: PhantomData<P>,
) -> impl Func1<(CL, CR, CC), (CL, CR, CO)>
where
    CL: ContextGet<P>,
    CR: ContextGet<P>,
    CC: ContextSet<P, Set = CO>,
{
    move |(ca, cb, ci): (CL, CR, CC)| {
        let p = cb.get();
        (ca, cb, ci.set(p))
    }
}

pub fn mod_color_by_distance<CL, CR, CI, CO>((cl, cr, ci): (CL, CR, CI)) -> (CL, CR, CO)
where
    CL: ContextGet<Color<Vec3>>,
    CR: ContextGet<Distance<f32>>,
    CI: ContextSet<Color<Vec3>, Set = CO>,
{
    let Color(c) = cl.get();
    let Distance(d) = cr.get();
    (cl, cr, ci.set(Color(c * d)))
}

pub fn smooth_union<CL, CR, CC, CO>(
    k: impl Func1<f32, f32>,
) -> impl Func1<(CL, CR, CC), (CL, CR, CO)>
where
    CL: ContextGet<Distance<f32>>,
    CR: ContextGet<Distance<f32>>,
    CC: ContextSet<Distance<f32>, Set = CO>,
{
    move |(cl, cr, cc)| {
        let Distance(da) = cl.get();
        let Distance(db) = cr.get();
        let k = k(da);

        let dd = db - da;

        let t = (0.5 + 0.5 * dd / k).clamp(0.0, 1.0);
        let d = db.mix(da, t) - k * t * (1.0 - t);

        (cl, cr, cc.set(Distance(d)))
    }
}

pub fn smooth_union_prop<P, CL, CR, CC, CO>(
    _: PhantomData<P>,
    k: impl Func1<f32, f32>,
) -> impl Func1<(CL, CR, CC), (CL, CR, CO)>
where
    CL: ContextGet<Distance<f32>> + ContextGet<P>,
    CR: ContextGet<Distance<f32>> + ContextGet<P>,
    CC: ContextSet<P, Set = CO>,
    P: Mix<T = f32>,
{
    move |(ca, cb, ci): (CL, CR, CC)| {
        let Distance(da) = ca.get();
        let Distance(db) = cb.get();
        let k = k(da);

        let pa: P = ca.get();
        let pb: P = cb.get();

        let dd = db - da;
        let t = (0.5 + 0.5 * dd / k).clamp(0.0, 1.0);
        let p = pb.mix(pa, t);

        (ca, cb, ci.set(p))
    }
}

pub fn smooth_overlay<CL, CR, CC, CO>(
    k: impl Func1<f32, f32>,
) -> impl Func1<(CL, CR, CC), (CL, CR, CO)>
where
    CL: ContextGet<Distance<f32>>,
    CR: ContextGet<Distance<f32>>,
    CC: ContextSet<Distance<f32>, Set = CO>,
{
    move |(ca, cb, ci): (CL, CR, CC)| {
        let Distance(da) = ca.get();
        let Distance(db) = cb.get();
        let k = k(db);

        let t = (0.5 + 0.5 * db / k).clamp(0.0, 1.0);
        let d = db.mix(da, t) - k * t * (1.0 - t);

        (ca, cb, ci.set(Distance(d)))
    }
}

pub fn smooth_overlay_prop<P, CL, CR, CC, CO>(
    _: PhantomData<P>,
    k: impl Func1<f32, f32>,
) -> impl Func1<(CL, CR, CC), (CL, CR, CO)>
where
    CL: ContextGet<P>,
    CR: ContextGet<Distance<f32>> + ContextGet<P>,
    CC: ContextSet<P, Set = CO>,
    P: Mix<T = f32>,
{
    move |(ca, cb, ci): (CL, CR, CC)| {
        let Distance(db) = cb.get();
        let k = k(db);

        let pa: P = ca.get();
        let pb: P = cb.get();

        let t = (0.5 + 0.5 * db / k).clamp(0.0, 1.0);
        let p = pb.mix(pa, t);

        (ca, cb, ci.set(p))
    }
}

pub fn blend_union<CI, CO>(a: impl Func1<CI, CO>, b: impl Func1<CI, CO>) -> impl Func1<CI, CO>
where
    CI: Clone,
    CO: Clone + ContextGet<Distance<f32>>,
{
    move |c| {
        let (_, _, ci) = union((a(c.clone()), b(c), ()));
        ci
    }
}

pub fn blend_union_color<CI, LB, RB>(
    a: impl Func1<CI, LB>,
    b: impl Func1<CI, RB>,
) -> impl Func1<CI, Context<(), Distance<f32>, (), (), (), (), (), Color<Vec3>, ()>>
where
    LB: Clone + ContextGet<Distance<f32>> + ContextGet<Color<Vec3>>,
    RB: Clone + ContextGet<Distance<f32>> + ContextGet<Color<Vec3>>,
    CI: Clone,
{
    move |c| {
        let (_, _, ci) = {
            let (ca, cb, ci) = (a(c.clone()), b(c), Context::default());
            let Distance(da) = ca.get();
            let Distance(db) = cb.get();

            let Color(col_a) = ca.get();
            let Color(col_b) = cb.get();

            if da < db {
                (ca, cb, ci.set(Distance(da)).set(Color(col_a)))
            } else {
                (ca, cb, ci.set(Distance(db)).set(Color(col_b)))
            }
        };
        ci
    }
}
