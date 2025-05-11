use nalgebra::Complex;
use num_traits::Float;
use rand::{Rng, distr::uniform::SampleUniform};

use crate::Generator;

pub struct Aabb<T> {
    centre: Complex<T>,
    half_size: Complex<T>,
}

impl<T: Float> Aabb<T> {
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
    fn sample(&self, rng: &mut impl Rng) -> Complex<T> {
        let re = rng.random_range(T::from(-self.half_size.re).unwrap()..T::from(self.half_size.re).unwrap());
        let im = rng.random_range(T::from(-self.half_size.im).unwrap()..T::from(self.half_size.im).unwrap());
        self.centre + Complex::new(re, im)
    }
}
