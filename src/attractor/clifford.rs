//! Implementation of the Clifford attractor.

use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

/// The Clifford attractor defined by the equations:
/// - `x_{n+1} = sin(a * y_n) + c * cos(a * x_n)`
/// - `y_{n+1} = sin(b * x_n) + d * cos(b * y_n)`
#[derive(Debug, Clone, Copy)]
pub struct Clifford<T> {
    /// Parameter 'a' in the Clifford attractor equation.
    a: T,
    /// Parameter 'b' in the Clifford attractor equation.
    b: T,
    /// Parameter 'c' in the Clifford attractor equation.
    c: T,
    /// Parameter 'd' in the Clifford attractor equation.
    d: T,
}

impl<T> Clifford<T> {
    /// Creates a new `Clifford` attractor with the specified parameters.
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
