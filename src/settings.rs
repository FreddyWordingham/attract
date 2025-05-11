use num_traits::Float;
use std::marker::PhantomData;

use crate::{Attractor, Generator};

pub struct Settings<'a, T, A: Attractor<T>, G: Generator<T>> {
    pub attractor: &'a A,
    pub generator: &'a G,

    pub offset: [T; 2],
    pub scale: T,

    pub resolution: [usize; 2],
    pub num_samples: usize,
    pub num_groups: usize,

    pub max_iter: usize,
    pub warmup: usize,

    _precision: PhantomData<T>,
}

impl<'a, T: Float, A: Attractor<T>, G: Generator<T>> Settings<'a, T, A, G> {
    pub fn new(
        attractor: &'a A,
        generator: &'a G,
        offset: [T; 2],
        scale: T,
        resolution: [usize; 2],
        num_samples: usize,
        num_groups: usize,
        max_iter: usize,
        warmup: usize,
    ) -> Self {
        debug_assert!(scale > T::zero(), "Scale must be greater than 0");
        debug_assert!(resolution[0] > 0, "Resolution X must be greater than 0");
        debug_assert!(resolution[1] > 0, "Resolution Y must be greater than 0");
        debug_assert!(num_samples > 0, "Number of samples must be greater than 0");
        debug_assert!(num_groups > 0, "Number of groups must be greater than 0");
        debug_assert!(max_iter > 0, "Max iterations must be greater than 0");
        Self {
            attractor,
            generator,
            offset,
            scale,
            resolution,
            num_samples,
            num_groups,
            max_iter,
            warmup,
            _precision: PhantomData,
        }
    }
}
