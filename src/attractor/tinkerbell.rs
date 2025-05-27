//! Implementation of the Tinkerbell map.

use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

/// The Tinkerbell map defined by the equations:
/// - `x_{n+1} = x_n^2 - y_n^2 + a * x_n + b * y_n`
/// - `y_{n+1} = 2 * x_n * y_n + c * x_n + d * y_n`
#[derive(Debug, Clone, Copy)]
pub struct Tinkerbell<T> {
    /// Parameter 'a' in the Tinkerbell map equation.
    a: T,
    /// Parameter 'b' in the Tinkerbell map equation.
    b: T,
    /// Parameter 'c' in the Tinkerbell map equation.
    c: T,
    /// Parameter 'd' in the Tinkerbell map equation.
    d: T,
}

impl<T> Tinkerbell<T> {
    /// Creates a new `Tinkerbell` map with the specified parameters.
    #[inline]
    pub const fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
}

impl<T: Float + Copy> Attractor<T> for Tinkerbell<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        let x = p.re;
        let y = p.im;
        Complex::new(
            x * x - y * y + self.a * x + self.b * y,
            T::from(2.0).unwrap() * x * y + self.c * x + self.d * y,
        )
    }
}
