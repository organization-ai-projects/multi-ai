//rust_universal/src/ai/memory/persist.rs
// Fonctions utilitaires pour la persistance (sérialisation/désérialisation) générique.
use crate::utils::{file::PathWithExt, serialization};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

/// Gère les extensions activées pour la lecture et l'écriture.
pub(crate) struct ExtensionManager { // Visibilité restreinte à `memory`
    extensions: HashMap<&'static str, (bool, bool)>, // (lecture activée, écriture activée)
}

impl ExtensionManager {
    pub(crate) fn new() -> Self { // Visibilité restreinte à `memory`
        let mut extensions = HashMap::new();
        extensions.insert("bin", (true, true)); // Par défaut, bin activé en lecture et écriture
        extensions.insert("ron", (true, true)); // Par défaut, ron activé en lecture et écriture
        Self { extensions }
    }

    pub(crate) fn set_extension_status(&mut self, ext: &'static str, read: bool, write: bool) {
        self.extensions.insert(ext, (read, write));
    }

    pub(crate) fn is_read_enabled(&self, ext: &str) -> bool {
        self.extensions.get(ext).map_or(false, |(read, _)| *read)
    }

    pub(crate) fn is_write_enabled(&self, ext: &str) -> bool {
        self.extensions.get(ext).map_or(false, |(_, write)| *write)
    }

    pub(crate) fn get_read_enabled_extensions(&self) -> Vec<&str> {
        self.extensions
            .iter()
            .filter_map(|(ext, (read, _))| if *read { Some(*ext) } else { None })
            .collect()
    }

    pub(crate) fn get_write_enabled_extensions(&self) -> Vec<&str> {
        self.extensions
            .iter()
            .filter_map(|(ext, (_, write))| if *write { Some(*ext) } else { None })
            .collect()
    }

    pub(crate) fn validate(&self) -> io::Result<()> {
        for (ext, (read, write)) in &self.extensions {
            if *read && !*write {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Extension '{}' activée en lecture mais pas en écriture", ext),
                ));
            }
        }
        Ok(())
    }

    /// Définit le mode de priorité pour l'écriture, en privilégiant le format binaire.
    pub(crate) fn mode_prefer_bin(&mut self) {
        if let Some((_, write)) = self.extensions.get_mut("bin") {
            *write = true;
        }
        if let Some((_, write)) = self.extensions.get_mut("ron") {
            *write = false;
        }
    }
}

fn ensure_parent_dir(path: &str) -> io::Result<()> { // Privé, utilisé uniquement dans ce fichier
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

pub(crate) fn save_to_ron<T: Serialize>(value: &T, path: &str) -> io::Result<()> {
    serialization::save_serialized(value, path, |v| Ok(ron::ser::to_string(v)?.into_bytes()))
}

pub(crate) fn save_to_bin<T: Serialize>(value: &T, path: &str) -> io::Result<()> {
    serialization::save_serialized(value, path, |v| Ok(bincode_next::encode_to_vec(v, bincode_next::config::standard())?))
}

pub(crate) fn load_from_ron<T: DeserializeOwned>(path: &str) -> io::Result<T> {
    let data = fs::read_to_string(path)?;
    if data.trim().is_empty() {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Empty file"));
    }
    let value = ron::de::from_str(&data).map_err(|e| Error::new(ErrorKind::Other, e))?;
    Ok(value)
}

pub(crate) fn load_from_bin<T: DeserializeOwned>(path: &str) -> io::Result<T> {
    let data = fs::read(path)?;
    let value = bincode_next::decode_from_slice(&data, bincode_next::config::standard()).map(|(v, _)| v).map_err(|e| Error::new(ErrorKind::Other, e))?;
    Ok(value)
}

fn load_by_ext<T: DeserializeOwned>(path: &str, ext: &str) -> io::Result<T> { // Privé, utilisé uniquement dans ce fichier
    match ext {
        "ron" => load_from_ron(path),
        "bin" => load_from_bin(path),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Unsupported format")),
    }
}

pub(crate) fn auto_load<T: serde::de::DeserializeOwned>(
    path: &str,
    manager: &ExtensionManager,
) -> io::Result<T> {
    for ext in manager.get_read_enabled_extensions() {
        let path_ext = PathWithExt::new(path, ext);
        if path_ext.has_extension() {
            return load_by_ext(path, ext);
        }
    }

    for ext in manager.get_read_enabled_extensions() {
        let path_ext = PathWithExt::new(path, ext);
        let candidate = path_ext.with_extension();
        if Path::new(&candidate).exists() {
            return load_by_ext(&candidate, ext);
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "No supported file found with active extensions",
    ))
}

pub(crate) fn save_auto<T: Serialize>(
    value: &T,
    base_path: &str,
    manager: &ExtensionManager,
) -> io::Result<()> {
    manager.validate()?; // Vérifie la cohérence des extensions

    for ext in manager.get_write_enabled_extensions() {
        let path_ext = PathWithExt::new(base_path, ext);
        let full_path = path_ext.with_extension();

        match ext {
            "ron" => save_to_ron(value, &full_path)?,
            "bin" => save_to_bin(value, &full_path)?,
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Unsupported format")),
        }
    }
    Ok(())
}