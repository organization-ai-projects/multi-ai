pub struct BehaviorAnalysis {
    pub observed_patterns: Vec<String>,
}

impl BehaviorAnalysis {
    pub fn new(stdout: &Option<String>, stderr: &Option<String>) -> Self {
        let mut analysis = Self {
            observed_patterns: Vec::new(),
        };

        analysis.analyze_behavior(stdout, stderr);
        analysis
    }

    fn analyze_behavior(&mut self, stdout: &Option<String>, stderr: &Option<String>) {
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
}
