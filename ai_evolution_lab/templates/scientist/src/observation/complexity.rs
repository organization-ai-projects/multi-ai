use std::collections::HashMap;

pub struct ComplexityAnalysis {
    pub complexity_score: u32,
    pub symbol_frequencies: HashMap<String, u32>,
    pub struct_depth: u32,
    pub repetition_score: u32,
}

impl ComplexityAnalysis {
    pub fn new(code: &str) -> Self {
        let mut analysis = Self {
            complexity_score: 0,
            symbol_frequencies: HashMap::new(),
            struct_depth: 0,
            repetition_score: 0,
        };

        analysis.analyze_code(code);
        analysis
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
    }
}
