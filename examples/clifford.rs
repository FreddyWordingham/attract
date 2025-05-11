use attract::{Circle, Clifford, Settings, render};
use chromatic::{Colour, ColourMap, Lab};
use nalgebra::Complex;
use nav::Transform;
use photo::Image;

type Precision = f32;

const NUM_SAMPLES: usize = 100000;
const NUM_GROUPS: usize = 100;
const MAX_ITER: usize = 10000;
const WARMUP: usize = 1000;

const OFFSET: [Precision; 2] = [0.0, 0.0];
const SCALE: Precision = 6.0;

const RESOLUTION: [usize; 2] = [1000, 1000];
const APPLY_LOG: bool = true;
const COLOURS: [&str; 21] = [
    "#FFFFFF", "#000004", "#08051D", "#190C3E", "#2F0A5B", "#470B6A", "#5C126E", "#721A6E", "#87216B", "#9B2964", "#B1325A",
    "#C43C4E", "#D74B3F", "#E55C30", "#F06F20", "#F8870E", "#FC9F07", "#FBBA1F", "#F7D340", "#F1ED71", "#FCFFA4",
];

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "clifford.png";

fn main() {
    let a = 1.26;
    let b = -1.35;
    let c = 1.88;
    let d = -0.82;
    println!("Random parameters: a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    let attractor = Clifford::new(a, b, c, d);
    let generator = Circle::new(Complex::new(0.0, 0.0), 2.0);

    let settings = Settings::new(
        &attractor,
        &generator,
        OFFSET,
        SCALE,
        RESOLUTION,
        NUM_SAMPLES,
        NUM_GROUPS,
        MAX_ITER,
        WARMUP,
    );

    // Render the attractor
    let data = render(&settings);

    // Normalise
    let max = *data.iter().max().unwrap() as Precision;
    let data = if APPLY_LOG {
        data.mapv(|v| (v as Precision).ln().max(0.0) / (max as Precision).ln())
    } else {
        data.mapv(|v| v as Precision / max as Precision)
    };

    // Colourise
    let colours = COLOURS
        .into_iter()
        .map(|s| Lab::<Precision>::from_hex(s).unwrap())
        .collect::<Vec<_>>();
    let cmap = ColourMap::new_uniform(&colours);

    let img = Transform::Rotate90 * data.mapv(|v| cmap.sample(v as Precision));
    img.save(format!("{}/{}", OUTPUT_DIR, IMAGE_NAME)).unwrap();
}
