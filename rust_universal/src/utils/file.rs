use std::fs;
use std::io;
use std::path::Path;

pub struct PathWithExt<'a> {
    base_path: &'a str,
    ext: &'a str,
}

impl<'a> PathWithExt<'a> {
    pub fn new(path: &'a str, ext: &'a str) -> Self {
        Self {
            base_path: path,
            ext,
        }
    }

    pub fn has_extension(&self) -> bool {
        self.base_path.ends_with(&format!(".{}", self.ext))
    }

    pub fn with_extension(&self) -> String {
        if self.has_extension() {
            self.base_path.to_string()
        } else {
            format!("{}.{}", self.base_path, self.ext)
        }
    }

    /// Vérifie si l'extension du chemin est activée pour la lecture dans ExtensionManager.
    pub fn is_valid_supported_ext(&self, manager: &crate::ai::memory::persist::ExtensionManager) -> bool {
        self.get_extension()
            .map_or(false, |ext| manager.is_read_enabled(ext))
    }

    /// Extrait l'extension du chemin, si elle existe.
    pub fn get_extension(&self) -> Option<&str> {
        Path::new(self.base_path)
            .extension()
            .and_then(|ext| ext.to_str())
    }
}

pub fn ensure_parent_dir(path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}
