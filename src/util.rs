use crate::fn_traits::Func1;
use core::ops::Mul;

pub fn constant<T, U>(u: U) -> impl Func1<T, U>
where
    U: Copy,
{
    move |_: T| u
}

#[cfg(feature = "spirv-std")]
pub fn frag_width<T>(fac: T) -> impl Func1<T, T>
where
    T: spirv_std::float::Float,
{
    move |t| spirv_std::arch::fwidth(t) * fac
}

#[cfg(not(feature = "spirv-std"))]
pub fn frag_width<T>(fac: T) -> impl Func1<T, T>
where
    T: Copy + Mul<T, Output = T>,
{
    move |t| t * fac
}
