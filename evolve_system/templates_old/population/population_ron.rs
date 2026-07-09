use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;

impl Population {
    pub fn save_ron(&self, path: &str) -> std::io::Result<()> {
        let pretty = PrettyConfig::default();
        let ron_str = to_string_pretty(self, pretty).unwrap();
        std::fs::write(path, ron_str)?;
        Ok(())
    }

    pub fn load_ron(path: &str) -> std::io::Result<Self> {
        let ron_str = std::fs::read_to_string(path)?;
        Ok(from_str(&ron_str).unwrap())
    }
}
