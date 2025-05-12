//! Implementation of the `DeJong` attractor.

use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

/// The `DeJong` attractor defined by the equations:
/// - `x_{n+1} = a * sin(im_n) - b * cos(x_n)`
/// - `y_{n+1} = c * sin(x_n) - d * cos(y_n)`
#[derive(Debug, Clone, Copy)]
pub struct DeJong<T> {
    /// Parameter 'a' in the `DeJong` attractor equation.
    a: T,
    /// Parameter 'b' in the `DeJong` attractor equation.
    b: T,
    /// Parameter 'c' in the `DeJong` attractor equation.
    c: T,
    /// Parameter 'd' in the `DeJong` attractor equation.
    d: T,
}

impl<T> DeJong<T> {
    /// Creates a new `DeJong` attractor with the specified parameters.
    #[inline]
    pub const fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
}

impl<T: Float + Copy> Attractor<T> for DeJong<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        Complex::new(
            (self.a * p.im).sin() - (self.b * p.re).cos(),
            (self.c * p.re).sin() - (self.d * p.im).cos(),
        )
    }
}
