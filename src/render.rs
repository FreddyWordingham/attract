//! Rendering functions for attractors.
//!
//! This module provides functionality to render attractors by iterating their equations
//! and counting the density of points in a discretized grid.

use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::Complex;
use ndarray::Array2;
use num_traits::{Float, FromPrimitive, NumCast};
use rand::rng;
use rayon::prelude::*;
use std::sync::Arc;

use crate::{Attractor, Generator, Settings};

/// Create a lambda function to map a position in the complex plane to a pixel in the image.
#[inline]
fn create_position_to_pixel_mapper<T: Send + Sync + Float + NumCast>(
    offset: Complex<T>,
    scale: T,
    resolution: [usize; 2],
) -> impl Fn(&Complex<T>) -> Option<[usize; 2]> + Send + Sync {
    let y_res = T::from(resolution[0]).unwrap();
    let x_res = T::from(resolution[1]).unwrap();

    let aspect_ratio = x_res / y_res;

    let height = scale;
    let width = scale * aspect_ratio;
    let half_height = height / T::from(2.0).unwrap();
    let half_width = width / T::from(2.0).unwrap();

    let max_y = y_res - T::one();
    let max_x = x_res - T::one();

    move |p: &Complex<T>| {
        let shifted_re = p.re - offset.re;
        let shifted_im = p.im - offset.im;
        let y = ((half_height - shifted_im) / height) * max_y;
        let x = ((half_width - shifted_re) / width) * max_x;
        (y >= T::zero() && y < y_res && x >= T::zero() && x < x_res).then(|| [y.to_usize().unwrap(), x.to_usize().unwrap()])
    }
}

/// Multi-threaded rendering of the attractor.
///
/// # Panics
///
/// This function will not panic.
#[inline]
pub fn render<T, G: Sync + Generator<T>>(settings: &Settings<T, G>) -> Array2<u32>
where
    T: Float + NumCast + FromPrimitive + Send + Sync,
{
    let progress_bar = ProgressBar::new(u64::try_from(settings.num_samples).unwrap());
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos}/{len} samples ({percent}%) - {eta_precise} remaining")
            .unwrap()
            .progress_chars("\u{2588}\u{2593}\u{2592}\u{2591}"),
    );
    progress_bar.set_message("Rendering fractal...");
    let arc_progress_bar = Arc::new(progress_bar);

    // Parallelize the group rendering (each group is single-threaded).
    let group_counts: Vec<Array2<u32>> = (0..settings.num_groups)
        .into_par_iter()
        .map(|_group_index| render_group(settings, &Arc::clone(&arc_progress_bar)))
        .collect();
    arc_progress_bar.finish_with_message("Rendering complete!");

    let mut total_counts = Array2::zeros(settings.resolution);
    for counts in group_counts {
        total_counts += &counts;
    }
    total_counts
}

/// Single-threaded rendering of the attractor.
#[inline]
fn render_group<T, G: Sync + Generator<T>>(settings: &Settings<T, G>, progress_bar: &Arc<ProgressBar>) -> Array2<u32>
where
    T: Float + NumCast + FromPrimitive + Send + Sync,
{
    let offset = Complex::new(settings.offset[0], settings.offset[1]);
    let mapper = create_position_to_pixel_mapper(offset, settings.scale, settings.resolution);

    // Create a counts array for this group
    let mut counts = Array2::zeros(settings.resolution);
    let mut rng = rng();

    // Process samples sequentially
    for i in 0..(settings.num_samples / settings.num_groups) {
        let pos = settings.generator.sample(&mut rng);
        render_path(
            settings.attractor.as_ref(),
            &mapper,
            pos,
            settings.max_iter,
            settings.warmup,
            &mut counts,
        );

        // Update progress bar every 100 samples to avoid too frequent updates
        if i % 100 == 0 {
            progress_bar.inc(100);
        }
    }

    // Ensure we account for any remaining samples in the progress bar
    let remainder = (settings.num_samples / settings.num_groups) % 100;
    if remainder > 0 {
        progress_bar.inc(u64::try_from(remainder).unwrap());
    }

    counts
}

/// Capture the path of a single sample point.
#[inline]
fn render_path<T>(
    attractor: &(dyn Attractor<T> + Sync),
    mapper: impl Fn(&Complex<T>) -> Option<[usize; 2]>,
    start: Complex<T>,
    max_iter: usize,
    warmup: usize,
    counts: &mut Array2<u32>,
) where
    T: Float + NumCast + FromPrimitive,
{
    let mut pos = start;

    // Warmup phase - skip initial iterations to reach the attractor
    for _ in 0..warmup {
        pos = attractor.iterate(pos);
    }

    // Count phase
    for _ in 0..max_iter {
        pos = attractor.iterate(pos);
        if let Some([x, y]) = mapper(&pos) {
            counts[[x, y]] += 1;
        }
    }
}
