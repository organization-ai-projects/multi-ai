use std::path::Path;
use similar::{ChangeTag, TextDiff};
use std::fs;

pub struct CargoBackup {
    original: String,
    path: std::path::PathBuf,
}

impl CargoBackup {
    pub fn new(path: &Path) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self {
            original: content,
            path: path.to_owned(),
        })
    }

    pub fn save(&self) -> std::io::Result<()> {
        // La sauvegarde est déjà faite dans new()
        Ok(())
    }

    pub fn show_diff(&self) -> String {
        let current = fs::read_to_string(&self.path).unwrap_or_default();
        let diff = TextDiff::from_lines(&self.original, &current);

        let mut output = String::new();
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            output.push_str(&format!("{} {}", sign, change));
        }
        output
    }

    pub fn rollback(&self) -> std::io::Result<()> {
        fs::write(&self.path, &self.original)
    }
}
