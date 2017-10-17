use super::*;

// TODO: Consider generalizing to the entire family of XorShift generators

pub struct XorShiftParameters {
    a: u32,
    b: u32,
    c: u32
}

const DEFAULT_XORSHIFT128_PARAMETERS: XorShiftParameters = XorShiftParameters {
    a: 11,
    b: 8,
    c: 19
};

pub struct XorShift128 {
    parameters: XorShiftParameters,
    state: [u32; 4]
}

impl SeedableRng for XorShift128 {
    fn new(seed: u32) -> Self {
        Self::with_parameters(seed, DEFAULT_XORSHIFT128_PARAMETERS)
    }
}

impl ParameterizedRng<XorShiftParameters> for XorShift128 {
    fn with_parameters(seed: u32, p: XorShiftParameters) -> Self {
        let mut state = [0; 4];
        state[3] = seed;
        XorShift128 {
            state: state,
            parameters: p
        }
    }
}

impl Rng for XorShift128 {
    fn next_u32(&mut self) -> u32 {
        let mut t = self.state[3];

        t ^= t << self.parameters.a;
        t ^= t >> self.parameters.b;

        self.state[3] = self.state[2];
        self.state[2] = self.state[1];
        self.state[1] = self.state[0];

        t ^= self.state[0];
        t ^= self.state[0] >> self.parameters.c;

        self.state[0] = t;
        return t
    }
}

pub struct XorShiftStarParameters {
    xor: XorShiftParameters,
    multiplier: u64
}

const DEFAULT_XORSHIFT1024STAR_PARAMETERS: XorShiftStarParameters = XorShiftStarParameters {
    xor: XorShiftParameters {
        a: 31,
        b: 11,
        c: 30
    },
    multiplier: 1181783497276652981
};

pub struct XorShift1024Star {
    parameters: XorShiftStarParameters,
    state: [u64; 16],
    index: usize
}


impl SeedableRng for XorShift1024Star {
    fn new(seed: u32) -> Self {
        Self::with_parameters(seed, DEFAULT_XORSHIFT1024STAR_PARAMETERS)
    }
}

impl ParameterizedRng<XorShiftStarParameters> for XorShift1024Star {
    fn with_parameters(seed: u32, p: XorShiftStarParameters) -> Self {
        let mut state = [0; 16];
        state[0] = seed as u64;
        XorShift1024Star {
            parameters: p,
            state: state,
            index: 0
        }
    }
}

impl Rng for XorShift1024Star {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let s0 = self.state[self.index];
        self.index = (self.index + 1) % 16;
        let mut s1 = self.state[self.index];

        s1 ^= s1 << self.parameters.xor.a;
        self.state[self.index] = s1 ^ s0 ^ (s1 >> self.parameters.xor.b) ^ (s0 >> self.parameters.xor.b);

        self.state[self.index] * self.parameters.multiplier
    }
}