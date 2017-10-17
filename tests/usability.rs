extern crate turbulence;

use turbulence::Rng;

#[test]
fn fast_rng_available() {
    turbulence::fast_rng().next_u32();
}

#[test]
fn interior_rng_available() {
    turbulence::InteriorRng::new(turbulence::fast_rng()).next_u32();
}

#[test]
fn std_rng_available() {
    turbulence::rng().next_u32();
}

#[test]
fn thread_local_rng_available() {
    turbulence::thread_rng().next_u32();
}