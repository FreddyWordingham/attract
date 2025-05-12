//! Implementation of the `Henon` attractor.

use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

/// The `Henon` attractor defined by the equations:
/// - `x_{n+1} = 1 - a * x_n^2 + y_n`
/// - `y_{n+1} = b * x_n`
#[derive(Debug, Clone, Copy)]
pub struct Henon<T> {
    /// Parameter 'a' in the `Henon` attractor equation.
    a: T,
    /// Parameter 'b' in the `Henon` attractor equation.
    b: T,
}

impl<T> Henon<T> {
    /// Creates a new `Henon` attractor with the specified parameters.
    #[inline]
    pub const fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T: Float + Copy> Attractor<T> for Henon<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        Complex::new(T::one() - self.a * p.re * p.re + p.im, self.b * p.re)
    }
}
