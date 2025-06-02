#[derive(Debug)]
pub enum Safety {
    Safe,        // Code normal
    Dangerous,   // Utilisation de chemins sensibles
    Lethal      // Code mortel (unsafe, etc.)
}

pub struct SurvivalRules {
    lethal_patterns: Vec<&'static str>,
    dangerous_paths: Vec<&'static str>,
}

impl SurvivalRules {
    pub fn new() -> Self {
        Self {
            lethal_patterns: vec![
                "unsafe",        // Code non sécurisé = mort immédiate
            ],
            dangerous_paths: vec![
                "std::process::Command",  // Création de processus interdite
                "std::env::set",          // Modification env interdite
            ]
        }
    }

    pub fn contains_lethal_code(&self, genetic_code: &str) -> bool {
        self.lethal_patterns.iter().any(|p| genetic_code.contains(p))
    }

    fn contains_dangerous_operations(&self, genetic_code: &str) -> bool {
        self.dangerous_paths.iter().any(|p| genetic_code.contains(p))
    }

    pub fn assess_safety(&self, genetic_code: &str) -> Safety {
        if self.contains_lethal_code(genetic_code) {
            Safety::Lethal
        } else if self.contains_dangerous_operations(genetic_code) {
            Safety::Dangerous
        } else {
            Safety::Safe
        }
    }
}
