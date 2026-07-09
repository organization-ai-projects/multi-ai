use crate::execution::ExecutionResult;
use crate::life_form::LifeForm;
use crate::survival_rules::SurvivalRules;
use crate::memory::NatureMemory;

pub struct NaturalSelection {
    survival_rules: SurvivalRules,
    memory: NatureMemory,
}

impl NaturalSelection {
    pub fn new(survival_rules: SurvivalRules, memory: NatureMemory) -> Self {
        Self {
            survival_rules,
            memory,
        }
    }

    pub fn evaluate(&self, form: &LifeForm, result: &ExecutionResult) -> bool {
        // Critères de sélection naturelle

        // 1. Le code a-t-il survécu à l'exécution ?
        if !result.survived {
            return false;
        }

        // 2. Le code contient-il des comportements ou patterns intéressants ?
        if self.has_interesting_patterns(&form.source_code) {
            return true;
        }

        // 3. Le code respecte-t-il les règles de sécurité ?
        if self.is_safe(&form.source_code) {
            return true;
        }

        // 4. Dernier recours : survie minimale (1ms suffit)
        true
    }

    fn has_interesting_patterns(&self, code: &str) -> bool {
        // Utilise la mémoire pour vérifier les patterns connus
        let known_patterns = &self.memory.successful_patterns;
        known_patterns.iter().any(|pattern| code.contains(pattern))
    }

    fn is_safe(&self, code: &str) -> bool {
        // Utilise SurvivalRules pour évaluer la sécurité
        matches!(self.survival_rules.assess_safety(code), crate::survival_rules::Safety::Safe)
    }
}
