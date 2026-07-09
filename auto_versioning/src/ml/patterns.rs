use std::collections::HashMap;

pub struct PatternLearner {
    patterns: HashMap<String, PatternStats>,
    confidence_threshold: f64,
}

impl PatternLearner {
    pub fn learn_from_changes(&mut self, changes: &[ChangeRecord]) {
        for change in changes {
            // Mise à jour des statistiques
            self.update_stats(change);
            
            // Ajustement des seuils de confiance
            self.adjust_confidence_levels();
            
            // Identification de nouveaux patterns
            if let Some(new_pattern) = self.identify_new_pattern(change) {
                self.patterns.insert(new_pattern.signature.clone(), new_pattern);
            }
        }
    }

    pub fn get_impact_prediction(&self, content: &str) -> Prediction {
        let matches = self.find_matching_patterns(content);
        self.compute_weighted_prediction(matches)
    }
}
