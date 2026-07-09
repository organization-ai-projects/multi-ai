use serde::{de::DeserializeOwned, Serialize};
use std::io::{self, Error, ErrorKind};

pub fn save_serialized<T, F>(value: &T, path: &str, serialize_fn: F) -> io::Result<()>
where
    T: Serialize,
    F: FnOnce(&T) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>,
{
    super::file::ensure_parent_dir(path)?;
    let serialized = serialize_fn(value).map_err(|e| Error::new(ErrorKind::Other, e))?;
    std::fs::write(path, serialized)
}

pub fn deserialize_from_bytes<T, F>(bytes: &[u8], deserialize_fn: F) -> io::Result<T>
where
    T: DeserializeOwned,
    F: FnOnce(&[u8]) -> Result<T, Box<dyn std::error::Error + Send + Sync>>,
{
    deserialize_fn(bytes).map_err(|e| Error::new(ErrorKind::Other, e))
}
