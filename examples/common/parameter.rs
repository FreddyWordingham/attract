use num_traits::Float;
use serde::{Deserialize, Serialize};

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
