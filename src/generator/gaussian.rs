//! Implementation of a Gaussian generator.
//!
//! This generator samples points from a Gaussian (normal) distribution centered
//! at a specified point in the complex plane.

use nalgebra::Complex;
use num_traits::{Float, FloatConst};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::Generator;

/// Generator for sampling points from a Gaussian distribution.
#[derive(Debug, Clone, Copy)]
pub struct Gaussian<T> {
    /// Center point of the Gaussian distribution in the complex plane.
    centre: Complex<T>,
    /// Standard deviation of the Gaussian distribution.
    std_dev: T,
}

impl<T: Float> Gaussian<T> {
    /// Creates a new Gaussian generator with the specified center and standard deviation.
    #[inline]
    pub fn new(centre: Complex<T>, std_dev: T) -> Self {
        debug_assert!(std_dev > T::zero(), "Standard deviation must be positive");
        Self { centre, std_dev }
    }
}

impl<T: Float + FloatConst> Generator<T> for Gaussian<T>
where
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> Complex<T> {
        let u1: T = rng.random();
        let u2: T = rng.random();
        let r = (T::from(-2).unwrap() * u1.ln()).sqrt() * self.std_dev;
        let theta = T::TAU() * u2;
        let x = r * theta.cos();
        let y = r * theta.sin();
        self.centre + Complex::new(x, y)
    }
}
