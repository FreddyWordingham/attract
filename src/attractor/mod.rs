//! Module defining attractor traits and implementations.
//!
//! Attractors are mathematical systems that evolve over time and usually settle into
//! a specific pattern or set of states. This module provides the core interface for
//! implementing different types of attractors.

use nalgebra::Complex;

mod chirikov;
mod clifford;
mod de_jong;
mod duffing;
mod gingerbreadman;
mod henon;
mod ikdea;
mod tinkerbell;

pub use chirikov::Chirikov;
pub use clifford::Clifford;
pub use de_jong::DeJong;
pub use duffing::Duffing;
pub use gingerbreadman::Gingerbreadman;
pub use henon::Henon;
pub use ikdea::Ikeda;
pub use tinkerbell::Tinkerbell;

/// Trait defining the interface for attractor implementations.
pub trait Attractor<T> {
    /// Iterates the attractor function starting at the provided complex coordinate.
    fn iterate(&self, p: Complex<T>) -> Complex<T>;
}
