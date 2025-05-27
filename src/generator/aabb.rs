//! Implementation of an Axis-Aligned Bounding Box generator.
//!
//! This generator samples points uniformly from a rectangular region in the complex plane.

use nalgebra::Complex;
use num_traits::Float;
use rand::{Rng, distr::uniform::SampleUniform};

use crate::Generator;

/// Generator for sampling points from an axis-aligned bounding box in the complex plane.
#[derive(Debug, Clone, Copy)]
pub struct Aabb<T> {
    /// Center point of the bounding box in the complex plane.
    centre: Complex<T>,
    /// Half of the width and height of the bounding box.
    half_size: Complex<T>,
}

impl<T: Float> Aabb<T> {
    /// Creates a new `Aabb` generator with the specified center and half size.
    #[inline]
    pub fn new(centre: Complex<T>, half_size: Complex<T>) -> Self {
        debug_assert!(
            half_size.re > T::zero() && half_size.im > T::zero(),
            "Half size must be positive"
        );
        Self { centre, half_size }
    }
}

impl<T: Float + SampleUniform> Generator<T> for Aabb<T> {
    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> Complex<T> {
        let re = rng.random_range(T::from(-self.half_size.re).unwrap()..T::from(self.half_size.re).unwrap());
        let im = rng.random_range(T::from(-self.half_size.im).unwrap()..T::from(self.half_size.im).unwrap());
        self.centre + Complex::new(re, im)
    }
}
