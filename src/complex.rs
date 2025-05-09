use num_traits::Float;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Complex<T> {
    pub real: T,
    pub imag: T,
}

impl<T> Complex<T> {
    #[inline]
    pub const fn new(real: T, imag: T) -> Self {
        Self { real, imag }
    }
}

/// Negation
impl<T: Neg<Output = T> + Copy> Neg for Complex<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.real, -self.imag)
    }
}

/// Complex addition
impl<T: Copy + Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

/// Complex subtraction
impl<T: Copy + Sub<Output = T>> Sub for Complex<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Complex<f32> {
    #[must_use]
    #[inline]
    pub fn div_scalar(self, scalar: f32) -> Self {
        Self {
            real: self.real / scalar,
            imag: self.imag / scalar,
        }
    }

    #[must_use]
    #[inline]
    pub fn norm(&self) -> f32 {
        self.norm_sqr().sqrt()
    }

    #[must_use]
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        let r = self.norm();
        let theta = self.imag.atan2(self.real);
        let new_r = r.powf(n);
        let new_theta = theta * n;
        Self::new(new_r * new_theta.cos(), new_r * new_theta.sin())
    }
}

/// Complex division
impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Div for Complex<T> {
    type Output = Self;

    #[expect(
        clippy::suspicious_operation_groupings,
        reason = "Clippy false positive: this is not a bug."
    )]
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / (rhs.real * rhs.real + rhs.imag * rhs.imag),
            imag: (self.imag * rhs.real - self.real * rhs.imag) / (rhs.real * rhs.real + rhs.imag * rhs.imag),
        }
    }
}

/// Scalar multiplication
impl<T: Copy + Div<Output = T>> Div<T> for Complex<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

/// Complex multiplication
impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Mul for Complex<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

// Norm squared
impl<T: Copy + Add<Output = T> + Mul<Output = T>> Complex<T> {
    #[inline]
    pub fn norm_sqr(&self) -> T {
        self.real * self.real + self.imag * self.imag
    }
}

/// Integer power
impl<T: Float> Complex<T> {
    #[must_use]
    #[inline]
    pub fn powi(self, n: u32) -> Self {
        if n == 0 {
            return Self::new(T::one(), T::zero());
        }
        let mut result = self;
        for _ in 1..n {
            result = result * self;
        }
        result
    }
}

/// Absolute value
impl<T: Float> Complex<T> {
    #[inline]
    pub fn abs(self) -> T {
        self.norm_sqr().sqrt()
    }

    /// Reciprocal/inverse.
    #[must_use]
    #[inline]
    pub fn inv(self) -> Self {
        let norm = self.norm_sqr();
        Self::new(self.real / norm, -self.imag / norm)
    }
}
