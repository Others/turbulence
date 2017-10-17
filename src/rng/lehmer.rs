use super::*;

pub struct LehmerParameters {
    modulus: u32,
    multiplier: u32
}

pub const MINSTD: LehmerParameters = LehmerParameters {
    modulus: 2147483647, // TODO: Actually calculate this as 2^31 - 1
    multiplier: 16807 // TODO: Actually calculate this as 7^5
};

// See page 6 of http://www.firstpr.com.au/dsp/rand31/p105-crawford.pdf
pub const MINSTD_UPDATED: LehmerParameters = LehmerParameters {
    modulus: 2147483647, // TODO: Actually calculate this as 2^31 - 1
    multiplier: 48271
};

impl Default for LehmerParameters {
    fn default() -> Self {
        // TODO: Consider whether MINSTD_UPDATED should be the default
        MINSTD
    }
}

// TODO: Consider letting users create their own LehmerParameters (with checking)

pub struct LehmerRng {
    parameters: LehmerParameters,
    state: u32
}

impl SeedableRng for LehmerRng {
    fn new(seed: u32) -> Self {
        Self::with_parameters(seed, LehmerParameters::default())
    }
}

impl ParameterizedRng<LehmerParameters> for LehmerRng {
    fn with_parameters(seed: u32, p: LehmerParameters) -> Self {
        LehmerRng {
            parameters: p,
            state: seed
        }
    }
}

impl Rng for LehmerRng {
    fn next_u32(&mut self) -> u32 {
        self.state = self.parameters.multiplier.wrapping_mul(self.state) % self.parameters.modulus;
        self.state
    }
}