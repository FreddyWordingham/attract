use nalgebra::Complex;
use rand::Rng;

mod aabb;
mod circle;

pub use aabb::Aabb;
pub use circle::Circle;

pub trait Generator<T> {
    fn sample(&self, rng: &mut impl Rng) -> Complex<T>;
}
