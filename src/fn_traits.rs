pub trait Func<A>: Copy + Fn() -> A {}
impl<T, A> Func<A> for T where T: Copy + Fn() -> A {}

pub trait Func1<A, B>: Copy + Fn(A) -> B {}
impl<T, A, B> Func1<A, B> for T where T: Copy + Fn(A) -> B {}

pub trait Func2<A, B, C>: Copy + Fn(A, B) -> C {}
impl<T, A, B, C> Func2<A, B, C> for T where T: Copy + Fn(A, B) -> C {}

