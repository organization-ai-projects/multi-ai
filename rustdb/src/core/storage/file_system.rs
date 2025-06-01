use std::path::PathBuf;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use crate::core::error::Result;

pub struct FileStorage {
    root_path: PathBuf,
    page_size: usize,
}

impl FileStorage {
    pub fn new(root_path: PathBuf) -> Self {
        fs::create_dir_all(&root_path).unwrap_or_default();
        Self {
            root_path,
            page_size: 4096, // Taille standard de page
        }
    }

    pub fn read_page(&self, file_name: &str, page_id: u32) -> Result<Vec<u8>> {
        let mut file = self.open_file(file_name)?;
        let mut buffer = vec![0; self.page_size];
        file.seek(SeekFrom::Start((page_id as u64) * (self.page_size as u64)))?;
        file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_page(&self, file_name: &str, page_id: u32, data: &[u8]) -> Result<()> {
        let mut file = self.open_file(file_name)?;
        file.seek(SeekFrom::Start((page_id as u64) * (self.page_size as u64)))?;
        file.write_all(data)?;
        file.flush()?;
        Ok(())
    }

    fn open_file(&self, name: &str) -> io::Result<File> {
        let path = self.root_path.join(name);
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
    }
}
