use std::fs;
use std::path::Path;

pub struct ArchiveManager {
    base_dir: String,
}

impl ArchiveManager {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

    pub fn archive_dead_ai(&self, ai_path: &Path) -> std::io::Result<()> {
        let archive_dir = format!("{}/archives/dead_ais", self.base_dir);
        fs::create_dir_all(&archive_dir)?;

        let ai_name = ai_path.file_name().unwrap().to_str().unwrap();
        let archive_path = format!(
            "{}/{}_{}",
            archive_dir,
            ai_name,
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        fs::rename(ai_path, archive_path)?;
        Ok(())
    }

    pub fn archive_generation(
        &self,
        generation: usize,
        ais: Vec<std::path::PathBuf>,
    ) -> std::io::Result<()> {
        let archive_dir = format!("{}/archives/generation_{:05}", self.base_dir, generation);
        fs::create_dir_all(&archive_dir)?;

        for ai_path in ais {
            let dest = format!(
                "{}/{}",
                archive_dir,
                ai_path.file_name().unwrap().to_str().unwrap()
            );
            fs::copy(ai_path, dest)?;
        }
        Ok(())
    }
}
