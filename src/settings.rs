//! Configuration settings for attractor rendering.
//!
//! This module provides a structure to configure the parameters for rendering
//! attractors, including resolution, scale, and sampling methods.

use crate::{Attractor, Generator};

/// Configuration settings for rendering.
#[expect(
    clippy::exhaustive_structs,
    reason = "Settings struct is expected to be constructed directly."
)]
pub struct Settings<'a, T> {
    // Scientific parameters
    /// Reference to the attractor being rendered.
    pub attractor: Box<dyn Attractor<T> + Sync>,
    /// Reference to the generator used for sampling initial points.
    pub generator: &'a Generator<T>,

    // Rendering parameters
    /// Resolution of the output image [height, width].
    pub resolution: [usize; 2],
    /// Offset of the rendering viewport in the complex plane [real, imag].
    pub offset: [T; 2],
    /// Scale factor for the rendering viewport (scales imaginary axis directly, real axis is scaled by the aspect ratio set by `resolution`).
    pub scale: T,

    // Processing parameters
    /// Number of sample points to generate.
    pub num_samples: usize,
    /// Number of parallel groups for multi-threaded rendering.
    pub num_groups: usize,

    // Simulation parameters
    /// Maximum number of iterations per sample point.
    pub max_iter: usize,
    /// Number of warmup iterations before plotting point positions.
    pub warmup: usize,
}
