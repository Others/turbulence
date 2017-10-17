use super::{Rng, SeedableRng};

use super::rng::MersenneTwister19937;

pub struct StdRng {
    inner: MersenneTwister19937
}

impl SeedableRng for StdRng {
    fn new(seed: u32) -> Self {
        StdRng {
            inner: MersenneTwister19937::new(seed)
        }
    }
}

impl Rng for StdRng {
    // TODO: Figure out if there is a cleaner way to reimplement all these methods
    #[inline]
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.inner.fill_bytes(bytes)
    }

    #[inline]
    fn next_bool(&mut self) -> bool {
        self.inner.next_bool()
    }

    #[inline]
    fn next_u8(&mut self) -> u8 {
        self.inner.next_u8()
    }

    #[inline]
    fn next_i8(&mut self) -> i8 {
        self.inner.next_i8()
    }

    #[inline]
    fn next_u16(&mut self) -> u16 {
        self.inner.next_u16()
    }

    #[inline]
    fn next_i16(&mut self) -> i16 {
        self.inner.next_i16()
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    #[inline]
    fn next_i32(&mut self) -> i32 {
        self.inner.next_i32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    #[inline]
    fn next_i64(&mut self) -> i64 {
        self.inner.next_i64()
    }

    #[inline]
    fn next_f32(&mut self) -> f32 {
        self.inner.next_f32()
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        self.inner.next_f64()
    }
}
