use super::complexity::ComplexityAnalysis;
use super::behavior::BehaviorAnalysis;

pub struct SpecimenEvaluation {
    pub viability_score: f32,
    pub report: String,
}

impl SpecimenEvaluation {
    pub fn new(complexity: &ComplexityAnalysis, behavior: &BehaviorAnalysis) -> Self {
        let mut evaluation = Self {
            viability_score: 0.0,
            report: String::new(),
        };

        evaluation.evaluate(complexity, behavior);
        evaluation
    }

    fn evaluate(&mut self, complexity: &ComplexityAnalysis, behavior: &BehaviorAnalysis) {
        // Points pour la complexité
        self.viability_score += complexity.complexity_score as f32 * 0.5;

        // Points pour les patterns observés
        self.viability_score += behavior.observed_patterns.len() as f32 * 1.0;

        // Points pour la profondeur des structures
        self.viability_score += complexity.struct_depth as f32 * 2.0;

        // Réduction pour les répétitions excessives
        self.viability_score -= complexity.repetition_score as f32 * 0.2;

        // Génération du rapport
        self.report = format!(
            "Rapport d'analyse:\n\
            Complexité: {}\n\
            Profondeur des structures: {}\n\
            Score de répétition: {}\n\
            Patterns observés: {:?}\n\
            Score de viabilité: {:.2}",
            complexity.complexity_score,
            complexity.struct_depth,
            complexity.repetition_score,
            behavior.observed_patterns,
            self.viability_score
        );
    }
}
