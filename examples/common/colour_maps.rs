use chromatic::{Colour, ColourMap, Lab};
use num_traits::Float;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Colection of colour maps.
#[derive(Debug, Serialize, Deserialize)]
pub struct ColourMaps {
    pub colour_maps: HashMap<String, Vec<String>>,
}

impl ColourMaps {
    pub fn build<T: Float + Send + Sync>(&self, name: &str) -> ColourMap<Lab<T>, T, 3> {
        if let Some(colours) = self.colour_maps.get(name) {
            let colours: Vec<_> = colours.iter().map(|s| Lab::from_hex(s).unwrap()).collect();
            ColourMap::new_uniform(colours.as_slice())
        } else {
            panic!("Colour map '{}' not found", name);
        }
    }
}
