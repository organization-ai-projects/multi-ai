use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

// Structure représentant une variante d'un glyphe avec représentation bitmap et vectorielle
#[derive(Serialize, Deserialize, Clone)]
pub struct GlyphVariant {
    pub bitmap: String,
    pub vector: String,
    pub notes: Option<String>,
}

// Structure représentant les connaissances sur un glyphe spécifique
#[derive(Serialize, Deserialize, Clone)]
pub struct GlyphKnowledge {
    pub letter: char,
    pub variants: Vec<GlyphVariant>,
    pub chosen_index: Option<usize>,
}

// Cette structure représente une mémoire pour stocker des éléments graphiques et de mise en page
// tels que des glyphes, des dispositions, des styles, et des blocs de construction
#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    pub glyphs: HashMap<String, String>,
    pub glyph_knowledge: HashMap<char, GlyphKnowledge>,
    pub layouts: HashMap<String, (f32, f32)>,
    pub styles: HashMap<String, String>,
    pub builder_blocks: HashMap<String, String>,
}

impl Memory {
    pub fn load() -> Self {
        // Définir le chemin du dossier de mémoire
        let memory_dir = "unl_ia/memory";

        // Créer le dossier de mémoire s'il n'existe pas
        fs::create_dir_all(memory_dir).ok();

        // Priorité au format binaire
        if let Some(memory) = Self::load_bin(memory_dir) {
            return memory;
        }

        // Fallback au format RON
        Self::load_ron(memory_dir).unwrap_or_default()
    }

    fn load_bin(memory_dir: &str) -> Option<Self> {
        fs::read(format!("{}/memory.bin", memory_dir))
            .ok()
            .and_then(|data| bincode::deserialize(&data).ok())
    }

    fn load_ron(memory_dir: &str) -> Option<Self> {
        fs::read_to_string(format!("{}/memory.ron", memory_dir))
            .ok()
            .and_then(|data| ron::from_str(&data).ok())
    }

    pub fn save(&self) {
        // Définir le chemin du dossier de mémoire
        let memory_dir = "unl_ia/memory";

        // Sauvegarde dans les deux formats
        self.save_bin(memory_dir);
        self.save_ron(memory_dir);
    }

    fn save_bin(&self, memory_dir: &str) {
        if let Ok(data) = bincode::serialize(self) {
            fs::create_dir_all(memory_dir).ok();
            fs::write(format!("{}/memory.bin", memory_dir), data).ok();
        }
    }

    fn save_ron(&self, memory_dir: &str) {
        if let Ok(data) = ron::to_string(self) {
            fs::create_dir_all(memory_dir).ok();
            fs::write(format!("{}/memory.ron", memory_dir), data).ok();
        }
    }

    // Fusionne cette mémoire avec une autre, en conservant les données existantes
    pub fn merge(&mut self, other: &Memory) {
        // Fusionner les glyphes simples
        for (k, v) in &other.glyphs {
            self.glyphs.entry(k.clone()).or_insert_with(|| v.clone());
        }

        // Fusionner les connaissances de glyphes
        for (k, v) in &other.glyph_knowledge {
            match self.glyph_knowledge.get_mut(k) {
                Some(existing) => {
                    // Ajouter uniquement les variantes non existantes
                    for variant in &v.variants {
                        if !existing
                            .variants
                            .iter()
                            .any(|v| v.bitmap == variant.bitmap && v.vector == variant.vector)
                        {
                            existing.variants.push(variant.clone());
                        }
                    }

                    // Mettre à jour l'index choisi s'il n'y en a pas
                    if existing.chosen_index.is_none() {
                        existing.chosen_index = v.chosen_index;
                    }
                }
                None => {
                    // Ajouter la connaissance complète
                    self.glyph_knowledge.insert(*k, v.clone());
                }
            }
        }

        // Fusionner les layouts
        for (k, v) in &other.layouts {
            self.layouts.entry(k.clone()).or_insert(*v);
        }

        // Fusionner les styles
        for (k, v) in &other.styles {
            self.styles.entry(k.clone()).or_insert_with(|| v.clone());
        }

        // Fusionner les blocs builder
        for (k, v) in &other.builder_blocks {
            self.builder_blocks
                .entry(k.clone())
                .or_insert_with(|| v.clone());
        }
    }
}
