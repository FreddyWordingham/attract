use attract::{Attractor, Complex, render};
use chromatic::{Colour, ColourMap, Lab};
use nav::Transform;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
// use vista::{DisplayExt, DoubleJoined};
use photo::Image;

type Precision = f32;

const OUTPUT_DIR: &str = "output";

#[derive(Debug, Serialize, Deserialize)]
struct Parameters<T> {
    pub centre: [T; 2],
    pub scale: T,
    pub resolution: [u32; 2],
    pub super_samples: Option<u32>,

    pub start: [T; 2],
    pub radius: T,
    pub num_samples: u32,
    pub max_iter: u32,
    pub draw_after: u32,

    pub attractor: Attractor<T>,

    pub image_name: String,
    pub log: bool,
    pub gamma: T,
    pub colour_map: String,
}

fn read_input_args<Parameters>() -> Parameters
where
    for<'de> Parameters: Deserialize<'de>,
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <parameters file>", args[0]);
        std::process::exit(1);
    }
    let params_file = &args[1];
    let file_contents = read_to_string(&params_file).expect(&format!("Failed to read parameters file: {}", params_file));
    serde_yaml::from_str(&file_contents).expect(&format!("Failed to parse parameters file: {}", params_file))
}

fn main() {
    // Read parameters from file
    let params = read_input_args::<Parameters<Precision>>();

    // Render the attractor
    let data = render(
        Complex::new(params.centre[0], params.centre[1]),
        params.scale,
        [
            params.resolution[0] * params.super_samples.unwrap_or(1),
            params.resolution[1] * params.super_samples.unwrap_or(1),
        ],
        Complex::new(params.start[0], params.start[1]),
        params.radius,
        params.num_samples,
        params.max_iter,
        params.draw_after,
        &params.attractor,
    );

    // Normalise
    let max = *data.iter().max().unwrap() as Precision;
    let data = if params.log {
        data.mapv(|v| (v as Precision).ln().max(0.0) / (max as Precision).ln())
    } else {
        data.mapv(|v| v as Precision / max as Precision)
    };

    // Colourise
    let colours = vec![
        "#000004", "#08051D", "#190C3E", "#2F0A5B", "#470B6A", "#5C126E", "#721A6E", "#87216B", "#9B2964", "#B1325A",
        "#C43C4E", "#D74B3F", "#E55C30", "#F06F20", "#F8870E", "#FC9F07", "#FBBA1F", "#F7D340", "#F1ED71", "#FCFFA4",
    ]
    .into_iter()
    .map(|s| Lab::<Precision>::from_hex(s).unwrap())
    .collect::<Vec<_>>();
    let cmap = ColourMap::new_uniform(&colours);

    let img = Transform::Rotate90 * data.mapv(|v| cmap.sample(v as Precision));
    // println!("{}", img.display::<DoubleJoined>());
    img.save(format!("{}/{}", OUTPUT_DIR, params.image_name)).unwrap();
}
