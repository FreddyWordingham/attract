#![allow(unused_imports)]

mod attractor_builder;
mod colour_maps;
mod configuration;
mod generator_builder;
mod parameter;

pub mod prelude {
    pub use crate::common::{
        attractor_builder::AttractorBuilder,
        colour_maps::ColourMaps,
        configuration::{Configuration, PostProcessingSettings, ProcessingSettings, RenderingSettings, SimulationSettings},
        generator_builder::GeneratorBuilder,
        parameter::Parameter,
    };
}
