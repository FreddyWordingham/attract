//! Module defining generators for sampling initial points.
//!
//! Generators provide methods to sample initial points from different distributions,
//! which are then evolved through the attractor system.

use nalgebra::Complex;
use rand::Rng;

mod aabb;
mod circle;
mod gaussian;

pub use aabb::Aabb;
pub use circle::Circle;
pub use gaussian::Gaussian;

/// Trait defining the interface for generators.
pub trait Generator<T> {
    /// Samples a point from the generator.
    fn sample(&self, rng: &mut impl Rng) -> Complex<T>;
}
