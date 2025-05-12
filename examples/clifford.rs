use attract::{Clifford, Gaussian, Settings, render};
use chromatic::{Colour, ColourMap, Lab};
use nalgebra::Complex;
use nav::Transform;
use photo::Image;

#[path = "./common.rs"]
mod common;
use common::MAGMA as CMAP;

// Precision
type P = f32;

// Clifford attractor parameters
const A: P = -1.4;
const B: P = 1.6;
const C: P = 1.3;
const D: P = 0.7;

// Gaussian generator parameters
const CENTRE: [P; 2] = [0.0, 0.0];
const STD_DEV: P = 1.0;

// Rendering parameters
const RESOLUTION: [usize; 2] = [1000, 1000];
const OFFSET: [P; 2] = [0.0, 0.0];
const SCALE: P = 5.5;

// Post-processing parameters
const APPLY_LOG: bool = true;

// Processing parameters
const NUM_SAMPLES: usize = 100000;
const NUM_GROUPS: usize = 100;

// Simulation parameters
const MAX_ITER: usize = 10000;
const WARMUP: usize = 1000;

// Output parameters
const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "clifford.png";

fn main() {
    // Simulation settings
    let settings = Settings {
        attractor: &Clifford::new(A, B, C, D),
        generator: &Gaussian::new(Complex::new(CENTRE[0], CENTRE[1]), STD_DEV),

        resolution: RESOLUTION,
        offset: OFFSET,
        scale: SCALE,

        num_samples: NUM_SAMPLES,
        num_groups: NUM_GROUPS,

        max_iter: MAX_ITER,
        warmup: WARMUP,
    };

    // Render the attractor
    let data = &render(&settings);

    // Normalise samples
    let max = *data.iter().max().unwrap() as P;
    let data = if APPLY_LOG {
        data.mapv(|v| (v as P).ln().max(0.0) / (max as P).ln())
    } else {
        data.mapv(|v| v as P / max as P)
    };

    // Colourise
    let colours = CMAP.into_iter().map(|s| Lab::<P>::from_hex(s).unwrap()).collect::<Vec<_>>();
    let cmap = ColourMap::new_uniform(&colours);

    let img = Transform::Rotate90 * data.mapv(|v| cmap.sample(v as P));
    img.save(format!("{}/{}", OUTPUT_DIR, IMAGE_NAME)).unwrap();
}
