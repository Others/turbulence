use super::Rng;

pub trait Randomizable {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

pub trait RandomizableRange {
    fn default<R: Rng + ?Sized>(rng: &mut R, low: Self, high: Self) -> Self;

    fn inclusive<R: Rng + ?Sized>(rng: &mut R, low: Self, high: Self) -> Self;

    fn exclusive<R: Rng + ?Sized>(rng: &mut R, low: Self, high: Self) -> Self;
}

// Randomizeable implementations
impl Randomizable for u8 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u8()
    }
}

impl Randomizable for i8 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_i8()
    }
}

impl Randomizable for u16 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u16()
    }
}

impl Randomizable for i16 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_i16()
    }
}

impl Randomizable for u32 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u32()
    }
}

impl Randomizable for i32 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_i32()
    }
}

impl Randomizable for u64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u64()
    }
}

impl Randomizable for i64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_i64()
    }
}

impl Randomizable for f32 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_f32()
    }
}

impl Randomizable for f64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_f64()
    }
}

// RandomizableRange implementations