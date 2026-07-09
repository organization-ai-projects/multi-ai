use std::fmt;

pub trait CacheDebug: fmt::Debug {
    fn as_debug(&self) -> &dyn fmt::Debug;
}

impl<T: fmt::Debug> CacheDebug for T {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }
}
