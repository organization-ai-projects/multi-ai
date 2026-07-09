//! Utilitaires pour la manipulation de fichiers
//! 
//! Ce module fournit des traits et fonctions utilitaires pour simplifier
//! les opérations courantes sur le système de fichiers de manière cohérente.

use std::fs;
use std::io;
use std::path::Path;

/// Trait fournissant des méthodes utilitaires pour la manipulation de fichiers
pub(crate) trait FileStorageUtils {
    /// S'assure que le répertoire parent d'un chemin existe
    /// Crée tous les répertoires parents si nécessaire
    fn ensure_parent_dir(path: &str) -> io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    /// Vérifie si un chemin existe et est un fichier (pas un répertoire)
    fn file_exists(path: &str) -> bool {
        Path::new(path).exists() && Path::new(path).is_file()
    }
}
