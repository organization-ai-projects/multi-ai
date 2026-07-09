use super::types::Observation;
use std::collections::HashMap;
use uuid::uuid7;

pub struct SpecimenAnalysis {
    pub complexity_score: u32,
    pub observed_patterns: Vec<String>,
    pub symbol_frequencies: HashMap<String, u32>,
    pub struct_depth: u32,
    pub repetition_score: u32,
}

impl SpecimenAnalysis {
    pub fn new(code: &str) -> Self {
        let mut analysis = Self {
            complexity_score: 0,
            observed_patterns: Vec::new(),
            symbol_frequencies: HashMap::new(),
            struct_depth: 0,
            repetition_score: 0,
        };

        analysis.analyze_code(code);
        analysis
    }

    pub fn observe_surface_patterns(&self, code: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Base structures
        for pattern in ["fn", "let", "struct", "impl"] {
            if code.contains(pattern) {
                patterns.push(format!("motif_{}", pattern));
            }
        }

        // Nested structures
        let depth = code.chars().fold(0, |acc, c| {
            match c {
                '{' => acc + 1,
                '}' => acc - 1,
                _ => acc,
            }
        });
        patterns.push(format!("profondeur_{}", depth));

        patterns
    }

    fn analyze_code(&mut self, code: &str) {
        // Analyse des symboles
        for token in code.split_whitespace() {
            *self.symbol_frequencies.entry(token.to_string()).or_insert(0) += 1;
        }

        // Calcul de la profondeur des structures imbriquées
        let mut depth = 0;
        let mut max_depth = 0;
        for c in code.chars() {
            match c {
                '{' => {
                    depth += 1;
                    if depth > max_depth {
                        max_depth = depth;
                    }
                }
                '}' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
        self.struct_depth = max_depth;

        // Détection des motifs répétitifs
        let mut repetition_count = 0;
        let mut last_char = None;
        for c in code.chars() {
            if Some(c) == last_char {
                repetition_count += 1;
            }
            last_char = Some(c);
        }
        self.repetition_score = repetition_count as u32;

        // Calcul du score de complexité global
        self.complexity_score = 
            self.symbol_frequencies.len() as u32 +  // Diversité des symboles
            self.struct_depth * 3 +                // Poids plus élevé pour la profondeur
            self.repetition_score / 2;            // Réduction de l'impact des répétitions

        // Extraction des patterns observés
        self.observed_patterns = self.observe_surface_patterns(code);
    }

    pub fn worth_dissecting(&self, observation: &Observation) -> bool {
        let mut interest = 0;

        // Complexité élevée
        if self.complexity_score > 10 {
            interest += 2;
        }

        // Structures profondes
        if self.struct_depth > 2 {
            interest += 1;
        }

        // Motifs répétitifs nombreux
        if self.repetition_score > 5 {
            interest += 1;
        }

        // Patterns rares
        let unique_patterns = self.observed_patterns.len();
        if unique_patterns > 3 {
            interest += 1;
        }

        interest >= 3
    }

    pub fn analyze_behavior(&mut self, stdout: &Option<String>, stderr: &Option<String>) {
        if let Some(output) = stdout {
            // Analyse des logs pour détecter des comportements spécifiques
            if output.contains("error") {
                self.observed_patterns.push("behavior_error".to_string());
            }
            if output.contains("success") {
                self.observed_patterns.push("behavior_success".to_string());
            }
        }

        if let Some(errors) = stderr {
            // Analyse des erreurs pour détecter des anomalies
            if errors.contains("panic") {
                self.observed_patterns.push("behavior_panic".to_string());
            }
        }
    }

    pub fn evaluate_specimen(&self) -> f32 {
        let mut score = 0.0;

        // Points pour la complexité
        score += self.complexity_score as f32 * 0.5;

        // Points pour les patterns observés
        score += self.observed_patterns.len() as f32 * 1.0;

        // Points pour la profondeur des structures
        score += self.struct_depth as f32 * 2.0;

        // Réduction pour les répétitions excessives
        score -= self.repetition_score as f32 * 0.2;

        score
    }

    pub fn generate_report(&self) -> String {
        format!(
            "Rapport d'analyse:\n\
            Complexité: {}\n\
            Profondeur des structures: {}\n\
            Score de répétition: {}\n\
            Patterns observés: {:?}\n",
            self.complexity_score,
            self.struct_depth,
            self.repetition_score,
            self.observed_patterns
        )
    }

    pub fn to_observation(&self, code: &str) -> Observation {
        Observation {
            specimen_id: uuid7(),
            observation_type: super::types::ObservationType::External,
            is_alive: true,
            stdout: None,
            stderr: None,
            execution_time_ms: 0,
            source_code: Some(code.to_string()),
        }
    }
}
