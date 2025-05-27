//! Implementation of the Gingerbreadman map.

use nalgebra::Complex;
use num_traits::Float;
use std::marker::PhantomData;

use crate::Attractor;

/// The Gingerbreadman map defined by the equations:
/// - `x_{n+1} = 1 - y_n + |x_n|`
/// - `y_{n+1} = x_n`
#[derive(Debug, Clone, Copy)]
pub struct Gingerbreadman<T> {
    /// Phantom data to allow generic type T.
    _phantom: PhantomData<T>,
}

impl<T> Gingerbreadman<T> {
    /// Creates a new `Gingerbreadman` map.
    /// This map has no parameters.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<T: Float + Copy> Attractor<T> for Gingerbreadman<T> {
    #[inline]
    fn iterate(&self, p: Complex<T>) -> Complex<T> {
        let x = p.re;
        let y = p.im;
        Complex::new(T::one() - y + x.abs(), x)
    }
}
