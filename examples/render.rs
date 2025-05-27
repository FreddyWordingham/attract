use attract::{Settings, render};
use photo::Image;
use serde_yaml::from_str;
use std::{env::args, fs::read_to_string, process::exit};

#[path = "./common/mod.rs"]
mod common;
use common::prelude::*;

type Precision = f32;

const COLOUR_MAP_FILE: &str = "input/colour_maps.yaml";

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

    // Load colour maps
    let colour_maps_str = read_to_string(COLOUR_MAP_FILE).unwrap();
    let colour_maps: ColourMaps = from_str(&colour_maps_str).unwrap();
    let cmap = colour_maps.build(config.post_processing.colour_map.as_str());

    // Build the generator
    let generator = config.generator.build();

    // Calculate name length
    let filename_length = (config.num_frames - 1).to_string().len();

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
        let mut data = render(&settings);

        // If a transform is specified, apply it
        if let Some(transform) = &config.post_processing.transform {
            data = *transform * data;
        }

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
        let suffix = if config.num_frames > 1 {
            format!("_{:0width$}.png", n, width = filename_length)
        } else {
            ".png".to_string()
        };
        filename = filename.replace(".png", &suffix);
        img.save(filename).unwrap();
    }
}
