use std::time::{Duration, SystemTime};
use super::error::{Error, Result};

pub struct CacheEntry<T> {
    data: T,
    expires_at: SystemTime,
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            expires_at: SystemTime::now() + ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    pub fn get(&self) -> Result<&T> {
        if self.is_expired() {
            return Err(Error::CacheExpired);
        }
        Ok(&self.data)
    }
}
