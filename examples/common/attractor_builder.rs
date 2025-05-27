use attract::{Attractor, Chirikov, Clifford, DeJong, Duffing, Gingerbreadman, Henon, Ikeda, Tinkerbell};
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::common::parameter::Parameter;

/// Utility structure for building `Attractor` instances from deserialisable parameters.
#[derive(Debug, Serialize, Deserialize)]
pub enum AttractorBuilder<T> {
    Chirikov {
        k: Parameter<T>,
    },
    Clifford {
        a: Parameter<T>,
        b: Parameter<T>,
        c: Parameter<T>,
        d: Parameter<T>,
    },
    DeJong {
        a: Parameter<T>,
        b: Parameter<T>,
        c: Parameter<T>,
        d: Parameter<T>,
    },
    Duffing {
        a: Parameter<T>,
        b: Parameter<T>,
    },
    Gingerbreadman {},
    Henon {
        a: Parameter<T>,
        b: Parameter<T>,
    },
    Ikeda {
        u: Parameter<T>,
    },
    Tinkerbell {
        a: Parameter<T>,
        b: Parameter<T>,
        c: Parameter<T>,
        d: Parameter<T>,
    },
}

impl<T: 'static + Sync + Float> AttractorBuilder<T> {
    pub fn build(&self, index: usize, total: usize) -> Box<dyn Sync + Attractor<T>> {
        match self {
            AttractorBuilder::Chirikov { k } => Box::new(Chirikov::new(k.value(index, total))),
            AttractorBuilder::Clifford { a, b, c, d } => Box::new(Clifford::new(
                a.value(index, total),
                b.value(index, total),
                c.value(index, total),
                d.value(index, total),
            )),
            AttractorBuilder::DeJong { a, b, c, d } => Box::new(DeJong::new(
                a.value(index, total),
                b.value(index, total),
                c.value(index, total),
                d.value(index, total),
            )),
            AttractorBuilder::Duffing { a, b } => Box::new(Duffing::new(a.value(index, total), b.value(index, total))),
            AttractorBuilder::Gingerbreadman {} => Box::new(Gingerbreadman::new()),
            AttractorBuilder::Henon { a, b } => Box::new(Henon::new(a.value(index, total), b.value(index, total))),
            AttractorBuilder::Ikeda { u } => Box::new(Ikeda::new(u.value(index, total))),
            AttractorBuilder::Tinkerbell { a, b, c, d } => Box::new(Tinkerbell::new(
                a.value(index, total),
                b.value(index, total),
                c.value(index, total),
                d.value(index, total),
            )),
        }
    }
}
