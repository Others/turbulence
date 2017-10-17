use rand::{self, Rng};

pub struct OsRng {
    inner: rand::OsRng
}

impl OsRng {
    pub fn new() -> OsRng {
        OsRng{
            inner: rand::OsRng::new().expect("We should always be able to create an OsRng instance!")
        }
    }
}

impl super::Rng for OsRng {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }
}