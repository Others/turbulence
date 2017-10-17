use std::cell::RefCell;
use std::ops::DerefMut;

use super::{Randomizable, RandomizableRange, Rng};

// InteriorRng is just a helper when you want to hide that an Rng is mutable
pub struct InteriorRng<R: Rng> {
    inner: RefCell<R>
}

impl<R: Rng> InteriorRng<R> {
    pub fn new(rng: R) -> Self {
        InteriorRng {
            inner: RefCell::new(rng)
        }
    }

    // TODO: Figure out if there is a cleaner way to reimplement all these methods
    #[inline]
    pub fn fill_bytes(&self, bytes: &mut [u8]) {
        self.inner.borrow_mut().fill_bytes(bytes)
    }

    #[inline]
    pub fn next_bool(&self) -> bool {
        self.inner.borrow_mut().next_bool()
    }

    #[inline]
    pub fn next_u8(&self) -> u8 {
        self.inner.borrow_mut().next_u8()
    }

    #[inline]
    pub fn next_i8(&self) -> i8 {
        self.inner.borrow_mut().next_i8()
    }

    #[inline]
    pub fn next_u16(&self) -> u16 {
        self.inner.borrow_mut().next_u16()
    }

    #[inline]
    pub fn next_i16(&self) -> i16 {
        self.inner.borrow_mut().next_i16()
    }

    #[inline]
    pub fn next_u32(&self) -> u32 {
        self.inner.borrow_mut().next_u32()
    }

    #[inline]
    pub fn next_i32(&self) -> i32 {
        self.inner.borrow_mut().next_i32()
    }

    #[inline]
    pub fn next_u64(&self) -> u64 {
        self.inner.borrow_mut().next_u64()
    }

    #[inline]
    pub fn next_i64(&self) -> i64 {
        self.inner.borrow_mut().next_i64()
    }

    #[inline]
    pub fn next_f32(&self) -> f32 {
        self.inner.borrow_mut().next_f32()
    }

    #[inline]
    pub fn next_f64(&self) -> f64 {
        self.inner.borrow_mut().next_f64()
    }

    #[inline]
    pub fn random<T: Randomizable>(&self) -> T {
        T::random(self.inner.borrow_mut().deref_mut())
    }

    #[inline]
    pub fn range<T: RandomizableRange>(&self, start: T, end: T) -> T {
        T::default(self.inner.borrow_mut().deref_mut(), start, end)
    }

    #[inline]
    pub fn inclusive_range<T: RandomizableRange>(&self, start: T, end: T) -> T {
        T::inclusive(self.inner.borrow_mut().deref_mut(), start, end)
    }

    #[inline]
    pub fn exclusive_range<T: RandomizableRange>(&self, start: T, end: T) -> T {
        T::exclusive(self.inner.borrow_mut().deref_mut(), start, end)
    }
}


