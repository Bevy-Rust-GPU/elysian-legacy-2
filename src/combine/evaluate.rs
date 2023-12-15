use crate::fn_traits::Func1;

pub fn evaluate_left<FL, FR, LA, RA, LB, CI>(
    (fl, fr, ca, cb, ci): (FL, FR, LA, RA, CI),
) -> (FL, FR, LB, RA, CI)
where
    FL: Func1<LA, LB>,
{
    (fl, fr, fl(ca), cb, ci)
}

pub fn evaluate_right<FL, FR, LA, RA, RB, CI>(
    (fl, fr, ca, cb, ci): (FL, FR, LA, RA, CI),
) -> (FL, FR, LA, RB, CI)
where
    FR: Func1<RA, RB>,
{
    (fl, fr, ca, fr(cb), ci)
}

pub fn evaluate_both<FL, FR, LA, RA, LB, RB, CI>(
    input: (FL, FR, LA, RA, CI),
) -> (FL, FR, LB, RB, CI)
where
    FL: Func1<LA, LB>,
    FR: Func1<RA, RB>,
{
    evaluate_right(evaluate_left(input))
}

pub fn evaluate_end<FL, FR, CA, CB, CI>((_, _, _, _, ci): (FL, FR, CA, CB, CI)) -> CI {
    ci
}
