use nalgebra::Complex;

mod clifford;

pub use clifford::Clifford;

pub trait Attractor<T> {
    /// Iterates the attractor function starting at the provided complex coordinate.
    fn iterate(&self, p: Complex<T>) -> Complex<T>;
}
