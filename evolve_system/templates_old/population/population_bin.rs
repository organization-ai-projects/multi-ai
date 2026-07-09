use bincode::{deserialize, serialize};
use std::fs::File;
use std::io::{Read, Write};

impl Population {
    pub fn save_bincode(&self, path: &str) -> std::io::Result<()> {
        let encoded = serialize(self).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_bincode(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(deserialize(&buf).unwrap())
    }
}
