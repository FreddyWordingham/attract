//! Module defining generators for sampling initial points.
//!
//! Generators provide methods to sample initial points from different distributions,
//! which are then evolved through the attractor system.

use nalgebra::Complex;
use num_traits::{Float, FloatConst};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform, uniform::SampleUniform},
};

/// Complex number generators.
#[non_exhaustive]
pub enum Generator<T> {
    /// Samples points uniformly from an axis-aligned bounding box.
    Aabb {
        /// Center point of the bounding box in the complex plane.
        centre: Complex<T>,
        /// Half of the width and height of the bounding box.
        half_size: Complex<T>,
    },
    /// Samples points uniformly from a circular region.
    Circle {
        /// Center point of the circle in the complex plane.
        centre: Complex<T>,
        /// Radius of the circle.
        radius: T,
    },
    /// Samples points from a Gaussian distribution.
    Gaussian {
        /// Center point of the Gaussian distribution in the complex plane.
        centre: Complex<T>,
        /// Standard deviation of the Gaussian distribution.
        std_dev: T,
    },
}

/// Trait defining the interface for generators.
impl<T: Float + FloatConst + SampleUniform> Generator<T>
where
    StandardUniform: Distribution<T>,
{
    /// Samples a point from the generator.
    ///
    /// # Panics
    ///
    /// Panics if the standard deviation for a Gaussian generator is not positive,
    #[must_use]
    #[inline]
    pub fn sample<R: Rng>(&self, rng: &mut R) -> Complex<T> {
        match *self {
            Self::Aabb { centre, half_size } => {
                let re = rng.random_range(T::from(-half_size.re).unwrap()..T::from(half_size.re).unwrap());
                let im = rng.random_range(T::from(-half_size.im).unwrap()..T::from(half_size.im).unwrap());
                centre + Complex::new(re, im)
            }
            Self::Circle { centre, radius } => {
                let theta = rng.random_range(T::zero()..T::TAU());
                let rho = rng.random_range(T::zero()..radius).sqrt();
                let re = centre.re + rho * theta.cos();
                let im = centre.im + rho * theta.sin();
                centre + Complex::new(re, im)
            }
            Self::Gaussian { centre, std_dev } => {
                let u1: T = rng.random();
                let u2: T = rng.random();
                let r = (T::from(-2).unwrap() * u1.ln()).sqrt() * std_dev;
                let theta = T::TAU() * u2;
                let x = r * theta.cos();
                let y = r * theta.sin();
                centre + Complex::new(x, y)
            }
        }
    }
}
