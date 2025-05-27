//! Implementation of the Chirikov Standard Map.

use nalgebra::Complex;
use num_traits::Float;

use crate::Attractor;

/// The Chirikov Standard Map defined by the equations:
/// - `p_{n+1} = p_n + K * sin(x_n)`
/// - `x_{n+1} = x_n + p_{n+1}` (mod 2π)
///
/// Note: This implementation doesn't apply the modulo operation to keep
/// the output unbounded for visualization purposes.
#[derive(Debug, Clone, Copy)]
pub struct Chirikov<T> {
    /// Parameter 'K' controlling the strength of nonlinearity.
    k: T,
}

impl<T> Chirikov<T> {
    /// Creates a new `Chirikov` standard map with the specified parameter.
    #[inline]
    pub const fn new(k: T) -> Self {
        Self { k }
    }
}

impl<T: Float + Copy> Attractor<T> for Chirikov<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        let x = p.re; // position
        let p_momentum = p.im; // momentum

        let p_new = p_momentum + self.k * x.sin();
        let x_new = x + p_new;

        Complex::new(x_new, p_new)
    }
}
