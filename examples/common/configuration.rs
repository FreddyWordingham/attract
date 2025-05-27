use attract::Generator;
use nav::Transform;
use serde::{Deserialize, Serialize};

use crate::common::{attractor_builder::AttractorBuilder, generator_builder::GeneratorBuilder};

/// Configuration settings for rendering.
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderingSettings<T> {
    /// Resolution of the output image [height, width].
    pub resolution: [usize; 2],
    /// Offset of the rendering viewport in the complex plane [real, imag].
    pub offset: [T; 2],
    /// Scale factor for the rendering viewport (scales imaginary axis directly, real axis is scaled by the aspect ratio set by `resolution`).
    pub scale: T,
}

/// Configuration settings for processing.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingSettings {
    /// Number of sample points to generate.
    pub num_samples: usize,
    /// Number of parallel groups for multi-threaded rendering.
    pub num_groups: usize,
}

/// Configuration settings for rendering.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationSettings {
    /// Maximum number of iterations per sample point.
    pub max_iter: usize,
    /// Number of warmup iterations before plotting point positions.
    pub warmup: usize,
}

/// Configuration settings for post-processing.
#[derive(Debug, Serialize, Deserialize)]
pub struct PostProcessingSettings {
    /// Apply logarithmic scaling to the output.
    pub apply_log: bool,
    /// Colour map to use for rendering.
    pub colour_map: String,
    /// Output directory for saving images.
    pub output_dir: String,
    /// Name of the output image file.
    pub image_name: String,
    /// Transform to apply to the output image.
    pub transform: Option<Transform>,
}

/// Complete configuration settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration<T> {
    /// Attractor settings.
    pub attractor: AttractorBuilder<T>,
    /// Generator settings.
    pub generator: GeneratorBuilder<T>,
    /// Number of frames to generate.
    pub num_frames: usize,
    /// Rendering settings.
    pub rendering: RenderingSettings<T>,
    /// Processing settings.
    pub processing: ProcessingSettings,
    /// Simulation settings.
    pub simulation: SimulationSettings,
    /// Post-processing settings.
    pub post_processing: PostProcessingSettings,
}
