//! Implementation of the Duffing attractor.

use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

/// The Duffing attractor defined by the equations:
/// - `x_{n+1} = y_n`
/// - `y_{n+1} = -b * x_n + a * y_n - y_n^3`
#[derive(Debug, Clone, Copy)]
pub struct Duffing<T> {
    /// Parameter 'a' in the Duffing attractor equation.
    a: T,
    /// Parameter 'b' in the Duffing attractor equation.
    b: T,
}

impl<T> Duffing<T> {
    /// Creates a new `Duffing` attractor with the specified parameters.
    #[inline]
    pub const fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T: Float + Copy> Attractor<T> for Duffing<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        let x = p.re;
        let y = p.im;
        Complex::new(y, -self.b * x + self.a * y - y * y * y)
    }
}
