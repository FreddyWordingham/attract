//! Implementation of the Ikeda map.

use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

/// The Ikeda map defined by the equations:
/// - `t = 0.4 - 6.0 / (1 + x_n^2 + y_n^2)`
/// - `x_{n+1} = 1 + u * (x_n * cos(t) - y_n * sin(t))`
/// - `y_{n+1} = u * (x_n * sin(t) + y_n * cos(t))`
#[derive(Debug, Clone, Copy)]
pub struct Ikeda<T> {
    /// Parameter 'u' controlling the strength of the nonlinearity.
    u: T,
}

impl<T> Ikeda<T> {
    /// Creates a new `Ikeda` map with the specified parameter.
    #[inline]
    pub const fn new(u: T) -> Self {
        Self { u }
    }
}

impl<T: Float + Copy> Attractor<T> for Ikeda<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        let x = p.re;
        let y = p.im;

        let r_squared = x * x + y * y;
        let t = T::from(0.4).unwrap() - T::from(6.0).unwrap() / (T::one() + r_squared);

        let cos_t = t.cos();
        let sin_t = t.sin();

        Complex::new(T::one() + self.u * (x * cos_t - y * sin_t), self.u * (x * sin_t + y * cos_t))
    }
}
