use crate::vec3::Vec3;
use rand::{Rng, SeedableRng};

use core::ops::{Deref, DerefMut};

/// A shim that points to the global `rand::rngs::ThreadRng` instance. isn't safe for multi-threading.
///
/// This struct is created by [`thread_local()`](../struct.rand::rngs::ThreadRng.html#method.thread_local)
pub struct ThreadFastRng(*mut rand::rngs::SmallRng);

impl Deref for ThreadFastRng {
    type Target = rand::rngs::SmallRng;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for ThreadFastRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

pub trait FromRawPtr<T> {
    fn from_ptr(ptr: *mut T) -> Self;
}

impl FromRawPtr<rand::rngs::SmallRng> for ThreadFastRng {
    fn from_ptr(ptr: *mut rand::rngs::SmallRng) -> ThreadFastRng {
        ThreadFastRng(ptr)
    }
}

pub fn local_rng() -> ThreadFastRng {
    use std::cell::RefCell;
    thread_local! {
        pub static THREAD_FAST_RNG: RefCell<rand::rngs::SmallRng> = RefCell::new(rand::rngs::SmallRng::from_entropy());
    }
    let ptr = THREAD_FAST_RNG.with(|r| r.as_ptr());
    ThreadFastRng::from_ptr(ptr)
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = local_rng();
    let mut p: Vec3;
    loop {
        p = Vec3::new(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}
