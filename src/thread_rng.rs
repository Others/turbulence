use std::cell::RefCell;
use std::rc::Rc;

use super::{Rng, StdRng};

pub fn thread_rng() -> ThreadRng {
    thread_local! {
        static THREAD_RNG_KEY: Rc<RefCell<StdRng>> = Rc::new(RefCell::new(super::rng()))
    }

    ThreadRng{
        inner: THREAD_RNG_KEY.with(|r| r.clone())
    }
}

pub struct ThreadRng {
    inner: Rc<RefCell<StdRng>>
}

impl Rng for ThreadRng {
    // TODO: Figure out if there is a cleaner way to reimplement all these methods
    #[inline]
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.inner.borrow_mut().fill_bytes(bytes)
    }

    #[inline]
    fn next_bool(&mut self) -> bool {
        self.inner.borrow_mut().next_bool()
    }

    #[inline]
    fn next_u8(&mut self) -> u8 {
        self.inner.borrow_mut().next_u8()
    }

    #[inline]
    fn next_i8(&mut self) -> i8 {
        self.inner.borrow_mut().next_i8()
    }

    #[inline]
    fn next_u16(&mut self) -> u16 {
        self.inner.borrow_mut().next_u16()
    }

    #[inline]
    fn next_i16(&mut self) -> i16 {
        self.inner.borrow_mut().next_i16()
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.inner.borrow_mut().next_u32()
    }

    #[inline]
    fn next_i32(&mut self) -> i32 {
        self.inner.borrow_mut().next_i32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.inner.borrow_mut().next_u64()
    }

    #[inline]
    fn next_i64(&mut self) -> i64 {
        self.inner.borrow_mut().next_i64()
    }

    #[inline]
    fn next_f32(&mut self) -> f32 {
        self.inner.borrow_mut().next_f32()
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        self.inner.borrow_mut().next_f64()
    }
}