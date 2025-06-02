use std::path::PathBuf;
use std::fs;
use crate::life_form::LifeForm;

pub struct FileSystem {
    base_path: PathBuf,
}

impl FileSystem {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn init_structure(&self) {
        fs::create_dir_all(&self.base_path).ok();
        fs::create_dir_all(self.base_path.join("forms")).ok();
        fs::create_dir_all(self.base_path.join("fossils")).ok();
    }

    pub fn save_life_form(&self, life_form: &LifeForm) {
        let form_path = self.base_path.join("forms").join(&life_form.id);
        fs::create_dir_all(&form_path).ok();
        
        // Save source code
        fs::create_dir_all(form_path.join("src")).ok();
        fs::write(form_path.join("src/main.rs"), &life_form.source_code).ok();
        
        // Save state
        ron::ser::to_string(&life_form)
            .map(|s| fs::write(form_path.join("form.ron"), s))
            .ok();
    }

    pub fn get_form_path(&self, id: &str) -> PathBuf {
        self.base_path.join("forms").join(id)
    }
}
