#![allow(dead_code)]

use attract::{Attractor, Clifford, DeJong, Henon};
use num_traits::Float;
use serde::{Deserialize, Serialize};

pub const MAGMA: [&str; 20] = [
    "#000000", "#08051D", "#190C3E", "#2F0A5B", "#470B6A", "#5C126E", "#721A6E", "#87216B", "#9B2964", "#B1325A", "#C43C4E",
    "#D74B3F", "#E55C30", "#F06F20", "#F8870E", "#FC9F07", "#FBBA1F", "#F7D340", "#F1ED71", "#FCFFA4",
];
pub const VIRIDIS: [&str; 22] = [
    "#000000", "#440154", "#471365", "#482475", "#463480", "#414487", "#3b528b", "#355f8d", "#2f6c8e", "#2a788e", "#25848e",
    "#21918c", "#1e9c89", "#22a884", "#2fb47c", "#44bf70", "#5ec962", "#7ad151", "#9bd93c", "#bddf26", "#dfe318", "#fde725",
];
pub const GREYSCALE: [&str; 2] = ["#000000", "#FFFFFF"];

pub fn get_colour_map(name: &str) -> Result<&'static [&'static str], String> {
    match name {
        "magma" => Ok(&MAGMA),
        "viridis" => Ok(&VIRIDIS),
        "greyscale" => Ok(&GREYSCALE),
        _ => Err(format!("Colour map '{}' not found", name)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Parameter<T> {
    Fixed(T),
    Sweep([T; 2]),
}

impl<T: Float> Parameter<T> {
    pub fn value(&self, index: usize, total: usize) -> T {
        debug_assert!(total > 0, "Total must be greater than 0");
        debug_assert!(index < total, "Index out of bounds");
        match self {
            Parameter::Fixed(value) => *value,
            Parameter::Sweep([min, max]) => {
                let range = *max - *min;
                let step = range / (T::from(total).unwrap() - T::one()).max(T::one());
                *min + step * T::from(index).unwrap()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AttractorBuilder<T> {
    Clifford {
        a: Parameter<T>,
        b: Parameter<T>,
        c: Parameter<T>,
        d: Parameter<T>,
    },
    DeJong {
        a: Parameter<T>,
        b: Parameter<T>,
        c: Parameter<T>,
        d: Parameter<T>,
    },
    Henon {
        a: Parameter<T>,
        b: Parameter<T>,
    },
}

impl<T: 'static + Sync + Float> AttractorBuilder<T> {
    pub fn build(&self, index: usize, total: usize) -> Box<dyn Sync + Attractor<T>> {
        match self {
            AttractorBuilder::Clifford { a, b, c, d } => Box::new(Clifford::new(
                a.value(index, total),
                b.value(index, total),
                c.value(index, total),
                d.value(index, total),
            )),
            AttractorBuilder::DeJong { a, b, c, d } => Box::new(DeJong::new(
                a.value(index, total),
                b.value(index, total),
                c.value(index, total),
                d.value(index, total),
            )),
            AttractorBuilder::Henon { a, b } => Box::new(Henon::new(a.value(index, total), b.value(index, total))),
        }
    }
}

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
}

/// Complete configuration settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration<T> {
    /// Reference to the attractor being rendered.
    pub attractor: AttractorBuilder<T>,
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
