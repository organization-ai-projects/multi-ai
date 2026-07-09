use std::fs;
use std::path::Path;

pub struct ArchiveManager {
    pub base_dir: String,
}

impl ArchiveManager {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

    pub fn archive_generation(
        &self,
        generation_id: usize,
        files: Vec<&Path>,
    ) -> std::io::Result<()> {
        let archive_dir = format!("{}/generation_{}", self.base_dir, generation_id);
        fs::create_dir_all(&archive_dir)?;

        for file in files {
            let file_name = file.file_name().unwrap();
            let dest = Path::new(&archive_dir).join(file_name);
            fs::copy(file, dest)?;
        }

        Ok(())
    }
}
