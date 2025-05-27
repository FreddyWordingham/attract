use attract::Generator;
use nalgebra::Complex;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::common::parameter::Parameter;

/// Utility structure for building `Generator` instances from deserialisable parameters.
#[derive(Debug, Serialize, Deserialize)]
pub enum GeneratorBuilder<T> {
    Aabb { centre: [T; 2], half_size: [T; 2] },
    Circle { centre: [T; 2], radius: T },
    Gaussian { centre: [T; 2], std_dev: T },
}

impl<T: 'static + Sync + Float> GeneratorBuilder<T> {
    pub fn build(&self) -> Generator<T> {
        match self {
            GeneratorBuilder::Aabb { centre, half_size } => Generator::Aabb {
                centre: Complex::new(centre[0], centre[1]),
                half_size: Complex::new(half_size[0], half_size[1]),
            },
            GeneratorBuilder::Circle { centre, radius } => Generator::Circle {
                centre: Complex::new(centre[0], centre[1]),
                radius: *radius,
            },
            GeneratorBuilder::Gaussian { centre, std_dev } => Generator::Gaussian {
                centre: Complex::new(centre[0], centre[1]),
                std_dev: *std_dev,
            },
        }
    }
}
