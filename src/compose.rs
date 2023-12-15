use crate::fn_traits::Func1;

#[macro_export]
macro_rules! compose {
    ($in:expr => $expr:expr $(,)?) => {
        $expr($in)
    };
    ($in:expr => $expr:expr, $($rest:expr),* $(,)?) => {
        compose!($expr($in) => $($rest),*)
    };
    ($($expr:expr),* $(,)?) => {
        move |c| compose!(c => $($expr),*)
    };
}

#[macro_export]
macro_rules! fold {
    ($f:expr => ($a:expr, $b:expr $(,)?)) => {
        $f($a, $b)
    };
    ($f:expr => ($a:expr, $($rest:expr),* $(,)?)) => {
        $f($a, fold!($f => ($($rest),*)))
    };
}

pub trait Compose<IN, OUT> {
    fn compose(self, input: IN) -> OUT;
}

impl<FA, A, B> Compose<A, B> for (FA,)
where
    FA: Copy + Fn(A) -> B,
{
    fn compose(self, input: A) -> B {
        let (fa,) = self;
        fa(input)
    }
}

impl<FA, FB, A, B, C> Compose<A, C> for (FA, FB)
where
    FA: Copy + Fn(A) -> B,
    FB: Copy + Fn(B) -> C,
{
    fn compose(self, input: A) -> C {
        let (fa, fb) = self;
        fb(fa(input))
    }
}

impl<FA, FB, FC, A, B, C, D> Compose<A, D> for (FA, FB, FC)
where
    FA: Copy + Fn(A) -> B,
    FB: Copy + Fn(B) -> C,
    FC: Copy + Fn(C) -> D,
{
    fn compose(self, input: A) -> D {
        let (fa, fb, fc) = self;
        fc(fb(fa(input)))
    }
}

impl<FA, FB, FC, FD, A, B, C, D, E> Compose<A, E> for (FA, FB, FC, FD)
where
    FA: Copy + Fn(A) -> B,
    FB: Copy + Fn(B) -> C,
    FC: Copy + Fn(C) -> D,
    FD: Copy + Fn(D) -> E,
{
    fn compose(self, input: A) -> E {
        let (fa, fb, fc, fd) = self;
        fd(fc(fb(fa(input))))
    }
}

impl<FA, FB, FC, FD, FE, A, B, C, D, E, F> Compose<A, F> for (FA, FB, FC, FD, FE)
where
    FA: Copy + Fn(A) -> B,
    FB: Copy + Fn(B) -> C,
    FC: Copy + Fn(C) -> D,
    FD: Copy + Fn(D) -> E,
    FE: Copy + Fn(E) -> F,
{
    fn compose(self, input: A) -> F {
        let (fa, fb, fc, fd, fe) = self;
        fe(fd(fc(fb(fa(input)))))
    }
}
