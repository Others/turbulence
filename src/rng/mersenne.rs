use super::*;

pub struct MersenneTwister19937 {
    state: [u32; N],
    index: usize
}

// Constants for MT19937 (a lot of constants...)
const N: usize = 624;

const F: u32 = 1812433253;
const M: usize = 397;

const R: u32 = 31;

const A: u32 = 0x9908B0DF;

const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;

const S: u32 = 7;
const B: u32 = 0x9D2C5680;

const T: u32 = 15;
const C: u32 = 0xEFC60000;

const L: u32 = 18;

// Calculated constants
const LOWER_MASK: u32 = (1 << R) - 1;
const UPPER_MASK: u32 = !LOWER_MASK;

impl SeedableRng for MersenneTwister19937 {
    fn new(seed: u32) -> Self {
        let mut state = [0; N];
        state[0] = seed;
        for i in 1..N {
            // 30 = 32 - 2
            state[i] = F.wrapping_mul(state[i-1] ^ (state[i-1] >> (32 - 2))) + (i as u32);
        }

        MersenneTwister19937 {
            state: state,
            index: N
        }
    }
}

impl MersenneTwister19937 {
    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.state[i] & UPPER_MASK).wrapping_add(self.state[(i + 1) % N] & LOWER_MASK);

            let mut x_a = x >> 1;
            if x % 2 != 0 {
                x_a = x_a ^ A;
            }

            self.state[i] = self.state[(i + M) % N] ^ x_a;
        }
        self.index = 0;
    }
}

impl Rng for MersenneTwister19937 {
    fn next_u32(&mut self) -> u32 {
        if self.index >= N {
            self.twist();
        }

        let mut y = self.state[self.index];
        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);

        self.index += 1;
        return y;
    }
}

