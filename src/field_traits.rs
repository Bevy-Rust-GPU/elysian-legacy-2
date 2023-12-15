use crate::{
    context::{Color, Distance, Gradient, Light, Position, Support, Tangent, Uv, ContextGet, ContextSet},
    fn_traits::Func1, lifting::lift_modify,
};
use rust_gpu_bridge::glam::{Vec2, Vec3};

pub trait PositionFunction<I> {
    fn pos(i: I) -> Position<f32>;
}

pub trait DistanceFunction<I> {
    fn dist(i: I) -> Distance<f32>;
}

pub trait GradientFunction<I> {
    fn grad(i: I) -> Gradient<Vec2>;
}

pub trait UvFunction<I> {
    fn uv(i: I) -> Uv<Vec2>;
}

pub trait TangentFunction<I> {
    fn tangent(i: I) -> Tangent<Vec2>;
}

pub trait ColorFunction<I> {
    fn color(i: I) -> Color<Vec3>;
}

pub trait LightFunction<I> {
    fn light(i: I) -> Light<f32>;
}

pub trait SupportFunction<I> {
    fn support(i: I) -> Support<Vec2>;
}

pub trait PositionClosure<T, U> {
    fn pos(i: T) -> impl Func1<U, Position<Vec2>>;
}

pub trait DistanceClosure<T, U> {
    fn dist(i: T) -> impl Func1<U, Distance<f32>>;
}

pub trait GradientClosure<T, U> {
    fn grad(i: T) -> impl Func1<U, Gradient<Vec2>>;
}

pub trait UvClosure<T, U> {
    fn uv(i: T) -> impl Func1<U, Uv<Vec2>>;
}

pub trait TangentClosure<T, U> {
    fn tangent(i: T) -> impl Func1<U, Tangent<Vec2>>;
}

pub trait ColorClosure<T, U> {
    fn color(i: T) -> impl Func1<U, Color<Vec3>>;
}

pub trait LightClosure<T, U> {
    fn light(i: T) -> impl Func1<U, Light<f32>>;
}

pub trait SupportClosure<T, U> {
    fn support(i: T) -> impl Func1<U, Support<Vec2>>;
}

pub trait Field<D, T, CA, CB> {
    fn field(t: T) -> impl Func1<CA, CB>;
}

