use attract::{Gaussian, Settings, render};
use chromatic::{Colour, ColourMap, Lab};
use nalgebra::Complex;
use photo::Image;
use serde_yaml::from_str;
use std::{env::args, fs::read_to_string, process::exit};

#[path = "./common.rs"]
mod common;
use common::*;

type Precision = f32;

fn read_parameters_filepath() -> String {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <parameters filepath>", args[0]);
        exit(1);
    }
    args[1].to_string()
}

fn main() {
    // Read input configuration
    let config_filepath = read_parameters_filepath();
    let config_str = read_to_string(config_filepath).unwrap();
    let config = from_str::<Configuration<Precision>>(&config_str).unwrap();

    // Build reusable objects
    let generator = Gaussian::new(Complex::new(0.0, 0.0), 1.0);
    let colours = get_colour_map(config.post_processing.colour_map.as_str())
        .unwrap()
        .iter()
        .map(|s| Lab::<Precision>::from_hex(s).unwrap())
        .collect::<Vec<_>>();
    let cmap = ColourMap::new_uniform(&colours);

    // Generate each frame
    for n in 0..config.num_frames {
        // Setup simulation settings
        let attractor = config.attractor.build(n, config.num_frames);
        let settings = Settings {
            attractor: attractor,
            generator: &generator,
            resolution: config.rendering.resolution,
            offset: config.rendering.offset,
            scale: config.rendering.scale,
            num_samples: config.processing.num_samples,
            num_groups: config.processing.num_groups,
            max_iter: config.simulation.max_iter,
            warmup: config.simulation.warmup,
        };

        // Render the attractor
        let data = &render(&settings);

        // Normalise samples
        let max = *data.iter().max().unwrap() as Precision;
        let data = if config.post_processing.apply_log {
            data.mapv(|v| (v as Precision).ln().max(0.0) / (max as Precision).ln())
        } else {
            data.mapv(|v| v as Precision / max as Precision)
        };

        // Colourise
        let img = data.mapv(|v| cmap.sample(v as Precision));

        // Save the image
        let mut filename = format!("{}/{}", config.post_processing.output_dir, config.post_processing.image_name);
        filename = filename.replace(".png", &format!("_{:06}.png", n));
        img.save(filename).unwrap();
    }
}
