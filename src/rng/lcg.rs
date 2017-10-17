use super::*;

pub struct LcgParameters {
    multiplier: u32,
    increment: u32
}

pub const BORLAND_PARAMETERS: LcgParameters = LcgParameters {
    multiplier: 22695477,
    increment: 1
};

pub const NUMERICAL_RECIPIES_PARAMETERS: LcgParameters = LcgParameters {
    multiplier: 1664525,
    increment: 1013904223
};


impl LcgParameters {
    pub fn new(multiplier: u32, increment: u32) -> Self {
        LcgParameters {
            multiplier: multiplier,
            increment: increment
        }
    }
}

impl Default for LcgParameters {
    fn default() -> Self {
        BORLAND_PARAMETERS
    }
}

pub struct Lcg {
    state: u32,
    parameters: LcgParameters
}

impl SeedableRng for Lcg {
    fn new(seed: u32) -> Self {
        Self::with_parameters(seed, LcgParameters::default())
    }
}

impl ParameterizedRng<LcgParameters> for Lcg {
    fn with_parameters(seed: u32, p: LcgParameters) -> Self {
        Lcg {
            state: seed,
            parameters: p
        }
    }
}

impl Rng for Lcg {
    fn next_u32(&mut self) -> u32 {
        self.state = self.parameters.multiplier.wrapping_mul(self.state).wrapping_add(self.parameters.increment);
        return self.state;
    }
}


