use serde_json;

impl Population {
    pub fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_json(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json).unwrap())
    }
}
