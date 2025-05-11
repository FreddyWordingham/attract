use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

pub struct Clifford<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

impl<T> Clifford<T> {
    #[inline]
    pub const fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
}

impl<T: Float + Copy> Attractor<T> for Clifford<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        Complex::new(
            (self.a * p.im).sin() + self.c * (self.a * p.re).cos(),
            (self.b * p.re).sin() + self.d * (self.b * p.im).cos(),
        )
    }
}
