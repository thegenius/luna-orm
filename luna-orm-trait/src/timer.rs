use std::marker::PhantomData;
use std::{borrow::Cow, time::Instant};
use tracing::debug;

#[cfg(debug_assertions)]
pub struct Timer<'a> {
    func_name: Cow<'a, str>,
    start: Instant,
}

#[cfg(debug_assertions)]
impl<'a> Timer<'a> {
    pub fn new(func_name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            func_name: func_name.into(),
            start: Instant::now(),
        }
    }
}

#[cfg(debug_assertions)]
impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        debug!(target: "luna_orm", fn_name= ?self.func_name, elapsed = ?elapsed);
    }
}

#[cfg(not(debug_assertions))]
pub struct Timer<'a> {
    _phantom: PhantomData<&'a ()>,
}

#[cfg(not(debug_assertions))]
impl<'a> Timer<'a> {
    pub fn new(func_name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
