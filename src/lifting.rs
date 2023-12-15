use crate::{
    context::{ContextGet, ContextSet},
    fn_traits::{Func, Func1},
};

/// Lift an I -> O into a C -> SetT<C, O>
pub fn lift_modify<I, O, CA, CB>(f: impl Func1<I, O>) -> impl Func1<CA, CB>
where
    CA: ContextGet<I> + ContextSet<O, Set = CB>,
{
    move |c: CA| {
        let p = c.get();
        c.set(f(p))
    }
}

/// Lift an -> O into a C -> SetT<C, O>
pub fn lift_replace<O, CA, CB>(f: impl Func<O>) -> impl Func1<CA, CB>
where
    CA: ContextSet<O, Set = CB>,
{
    move |c: CA| c.set(f())
}

pub fn lift_replace_1<O, CA, CB>(f: impl Func1<(), O>) -> impl Func1<CA, CB>
where
    CA: ContextSet<O, Set = CB>,
{
    move |c: CA| c.set(f(()))
}

// Lift a (LA, RA, CI) -> (LB, RB, CO) to (FL, FR, LA, RA, CI) -> (FL, FR, LB, RB, CO)
pub fn lift_blend_evaluate<FL, FR, LA, RA, LB, RB, CI, CO>(
    f: impl Func1<(LA, RA, CI), (LB, RB, CO)>,
) -> impl Func1<(FL, FR, LA, RA, CI), (FL, FR, LB, RB, CO)> {
    move |(fl, fr, la, ra, ci)| {
        let (lb, rb, co) = f((la, ra, ci));
        (fl, fr, lb, rb, co)
    }
}
