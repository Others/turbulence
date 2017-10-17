extern crate rand;

pub mod rng;
pub use rng::{Rng, SeedableRng, ParameterizedRng};

mod fast_rng;
pub use fast_rng::FastRng;

mod interior_rng;
pub use interior_rng::InteriorRng;

pub mod randomizable;
pub use randomizable::{Randomizable, RandomizableRange};

mod std_rng;
pub use std_rng::StdRng;

mod thread_rng;
pub use thread_rng::{ThreadRng, thread_rng};

pub fn fast_rng() -> FastRng {
    let seed = rng::os::OsRng::new().next_u32();
    FastRng::new(seed)
}

pub fn rng() -> StdRng {
    let seed = rng::os::OsRng::new().next_u32();
    StdRng::new(seed)
}


// TODO: Figure out if we want to implement this
//pub fn secure_rng() {
//
//}

// This code connects the Rng trait to the Randomizable trait
impl Rng {
    #[inline]
    pub fn random<T: Randomizable>(&mut self) -> T {
        T::random(self)
    }

    #[inline]
    pub fn range<T: RandomizableRange>(&mut self, start: T, end: T) -> T {
        T::default(self, start, end)
    }

    #[inline]
    pub fn inclusive_range<T: RandomizableRange>(&mut self, start: T, end: T) -> T {
        T::inclusive(self, start, end)
    }

    #[inline]
    pub fn exclusive_range<T: RandomizableRange>(&mut self, start: T, end: T) -> T {
        T::exclusive(self, start, end)
    }
}


