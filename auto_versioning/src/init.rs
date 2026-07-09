use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningMode {
    Local,
    Collaborative
}

pub fn first_run_setup() -> LearningMode {
    let theme = ColorfulTheme::default();
    
    println!("🤖 Configuration d'Auto-Versioning");
    println!("----------------------------------");
    
    let choices = &[
        "IA Locale (apprentissage uniquement sur vos projets)",
        "IA Collaborative (apprentissage partagé avec la communauté)",
    ];

    let selection = Select::with_theme(&theme)
        .with_prompt("Choisissez le mode d'apprentissage")
        .default(0)
        .items(choices.as_slice())
        .interact()
        .unwrap_or(0);

    if selection == 1 {
        println!("✨ Mode collaboratif activé - Merci de contribuer à l'amélioration du système!");
        LearningMode::Collaborative
    } else {
        println!("📱 Mode local activé - L'apprentissage reste sur votre machine");
        LearningMode::Local
    }
}

pub fn save_learning_mode(mode: &LearningMode, path: &PathBuf) -> std::io::Result<()> {
    let config = ron::to_string(&mode).unwrap();
    std::fs::write(path.join(".av-learning-mode"), config)
}
