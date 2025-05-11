use nalgebra::Complex;
use num_traits::{Float, FloatConst};
use rand::{Rng, distr::uniform::SampleUniform};

use crate::Generator;

pub struct Circle<T> {
    centre: Complex<T>,
    radius: T,
}

impl<T: Float> Circle<T> {
    #[inline]
    pub fn new(centre: Complex<T>, radius: T) -> Self {
        debug_assert!(radius > T::zero(), "Radius must be positive");
        Self { centre, radius }
    }
}

impl<T: Float + FloatConst + SampleUniform> Generator<T> for Circle<T> {
    #[inline]
    fn sample(&self, rng: &mut impl Rng) -> Complex<T> {
        let theta = rng.random_range(T::zero()..T::TAU());
        let rho = rng.random_range(T::zero()..self.radius).sqrt();
        let re = self.centre.re + rho * theta.cos();
        let im = self.centre.im + rho * theta.sin();
        self.centre + Complex::new(re, im)
    }
}
