#![feature(test)]
extern crate test;
use test::Bencher;

extern crate turbulence;
use turbulence::{Rng, SeedableRng};

use turbulence::rng::lcg::Lcg;
use turbulence::rng::lehmer::LehmerRng;
use turbulence::rng::mersenne::MersenneTwister19937;
use turbulence::rng::xorshift::{XorShift128, XorShift1024Star};
use turbulence::rng::os::OsRng;

const SEED: u32 = 281273839;

const MEGABYTE_BYTES: u32 = 1000^2;
const MEGABYTE_COUNT: u32 = 100;

const BYTE_COUNT: u32 = MEGABYTE_BYTES * MEGABYTE_COUNT;

const U32_WORDS: u32 = BYTE_COUNT/4;
const U64_WORDS: u32 = BYTE_COUNT/8;


fn u32_throughput<R: Rng>(rng: &mut R) -> u32 {
    let mut result = 0;
    for _ in 0..U32_WORDS {
        result ^= rng.next_u32()
    }
    result
}

#[bench]
fn lcg_u32_throughput(b: &mut Bencher) {
    let mut rng = Lcg::new(SEED);
    b.iter(|| u32_throughput(&mut rng));
}

#[bench]
fn lehmer_u32_throughput(b: &mut Bencher) {
    let mut rng = LehmerRng::new(SEED);
    b.iter(|| u32_throughput(&mut rng));
}


#[bench]
fn mersenne_u32_throughput(b: &mut Bencher) {
    let mut rng = MersenneTwister19937::new(SEED);
    b.iter(|| u32_throughput(&mut rng));
}

#[bench]
fn os_u32_throughput(b: &mut Bencher) {
    let mut rng = OsRng::new();
    b.iter(|| u32_throughput(&mut rng));
}


#[bench]
fn xorshift128_u32_throughput(b: &mut Bencher) {
    let mut rng = XorShift128::new(SEED);
    b.iter(|| u32_throughput(&mut rng));
}

#[bench]
fn xorshift1024star_u32_throughput(b: &mut Bencher) {
    let mut rng = XorShift1024Star::new(SEED);
    b.iter(|| u32_throughput(&mut rng));
}

fn u64_throughput<R: Rng>(rng: &mut R) -> u64 {
    let mut result = 0;
    for _ in 0..U64_WORDS {
        result ^= rng.next_u64()
    }
    result
}

#[bench]
fn lcg_u64_throughput(b: &mut Bencher) {
    let mut rng = Lcg::new(SEED);
    b.iter(|| u64_throughput(&mut rng));
}

#[bench]
fn lehmer_u64_throughput(b: &mut Bencher) {
    let mut rng = LehmerRng::new(SEED);
    b.iter(|| u64_throughput(&mut rng));
}


#[bench]
fn mersenne_u64_throughput(b: &mut Bencher) {
    let mut rng = MersenneTwister19937::new(SEED);
    b.iter(|| u64_throughput(&mut rng));
}

#[bench]
fn os_u64_throughput(b: &mut Bencher) {
    let mut rng = OsRng::new();
    b.iter(|| u64_throughput(&mut rng));
}


#[bench]
fn xorshift128_u64_throughput(b: &mut Bencher) {
    let mut rng = XorShift128::new(SEED);
    b.iter(|| u64_throughput(&mut rng));
}

#[bench]
fn xorshift1024star_u64_throughput(b: &mut Bencher) {
    let mut rng = XorShift1024Star::new(SEED);
    b.iter(|| u64_throughput(&mut rng));
}
