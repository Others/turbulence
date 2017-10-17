pub mod lcg;
pub use self::lcg::Lcg;

pub mod lehmer;
pub use self::lehmer::LehmerRng;

pub mod mersenne;
pub use self::mersenne::MersenneTwister19937;

pub mod os;
pub use self::os::OsRng;

pub mod xorshift;
pub use self::xorshift::{XorShift128, XorShift1024Star};

pub trait SeedableRng: Rng {
    fn new(seed: u32) -> Self;
}

pub trait ParameterizedRng<P>: Rng + SeedableRng {
    fn with_parameters(seed: u32, p: P) -> Self;
}

pub trait Rng {
    // TODO: Check whether the default impls hurt performance
    #[inline]
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        // TODO: Optimize this, by generating words instead of bytes
        for byte in bytes.iter_mut() {
            *byte = self.next_u8();
        }
    }

    #[inline]
    fn next_bool(&mut self) -> bool {
        self.next_u8() % 2 == 0
    }

    #[inline]
    fn next_u8(&mut self) -> u8 {
        self.next_u16() as u8
    }

    #[inline]
    fn next_i8(&mut self) -> i8 {
        self.next_u8() as i8
    }

    #[inline]
    fn next_u16(&mut self) -> u16 {
        self.next_u32() as u16
    }

    #[inline]
    fn next_i16(&mut self) -> i16 {
        self.next_u16() as i16
    }

    #[inline]
    fn next_u32(&mut self) -> u32;

    #[inline]
    fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | self.next_u32() as u64
    }

    #[inline]
    fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    #[inline]
    fn next_f32(&mut self) -> f32 {
        // Note: This algorithm is based on Java's, which is known to produce uniform results

        // Find 24 bit integer, and divide it by 2^24
        (self.next_u32() & 0xFFFFFF) as f32 / ((1 << 24) as f32)
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        // Note: This algorithm is based on Java's, which is known to produce uniform results

        // Find 53 bit integer, and divide it by 2^53
        (self.next_u64() & 0x1FFFFFFFFFFFFF) as f64 / (((1 as u64) << 53) as f64)
    }
}


// TODO: Figure out why this is broken
//impl<T: ParameterizedRng<P>, P: Default> SeedableRng for T {
//    fn new(seed: u32) -> Self {
//        Self::with_parameters(seed, P::default())
//    }
//}



