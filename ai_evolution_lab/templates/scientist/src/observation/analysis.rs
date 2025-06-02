use super::types::Observation;
use std::collections::{HashMap, HashSet};
use uuid::uuid7;

pub struct SpecimenAnalysis {
    pub complexity_score: u32,
    pub observed_patterns: Vec<String>,
    pub symbol_frequencies: HashMap<String, u32>,
    pub struct_depth: u32,
    pub repetition_score: u32
}

impl SpecimenAnalysis {
    pub fn new(code: &str) -> Self {
        let mut analysis = Self {
            complexity_score: 0,
            observed_patterns: Vec::new(),
            symbol_frequencies: HashMap::new(),
            struct_depth: 0,
            repetition_score: 0
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
                _ => acc
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

        // Calcul profondeur structures
        self.struct_depth = code.matches('{').count() as u32;

        // Détection motifs répétitifs
        let repeating = code.as_bytes()
            .windows(3)
            .filter(|w| w[0] == w[1] || w[1] == w[2])
            .count();
        self.repetition_score = repeating as u32;

        // Score complexité global
        self.complexity_score = 
            self.symbol_frequencies.len() as u32 +
            self.struct_depth * 2 +
            self.repetition_score;

        // Patterns observés
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

    pub fn to_observation(&self, code: &str) -> Observation {
        Observation {
            specimen_id: uuid7(),
            observation_type: super::types::ObservationType::External,
            is_alive: true,
            stdout: None,
            stderr: None,
            execution_time_ms: 0,
            source_code: Some(code.to_string())
        }
    }
}
