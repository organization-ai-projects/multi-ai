use std::path::Path;

pub fn get_project_root() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let parent_dir = manifest_dir.parent().unwrap();
    
    // Vérifie si le parent a un dossier src (cas sous-projet)
    if parent_dir.join("src").is_dir() {
        parent_dir.to_path_buf()
    } else {
        manifest_dir.to_path_buf()
    }
}