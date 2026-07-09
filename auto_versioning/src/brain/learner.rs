use super::model::{Impact, PatternStats, ErrorRecord, SuccessPattern}; // Corriger les importations
use super::feedback::{DevFeedback, GlobalLearning};
use chrono::Utc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct BrainLearner {
    pub(crate) brain: VersioningBrain,
    memory_path: PathBuf,
    collaborative: bool,
    global_learning: Option<GlobalLearning>,
}

impl BrainLearner {
    pub fn new(project_path: &std::path::Path) -> Self {
        let memory_path = project_path.join(".av-brain");
        let brain = Self::load_or_create_brain(&memory_path);
        
        Self { 
            brain, 
            memory_path, 
            collaborative: true,
            global_learning: None,
        }
    }

    pub fn new_local(path: &Path) -> Self {
        let mut learner = Self::new(path);
        learner.collaborative = false;
        learner
    }

    pub fn new_collaborative(path: &Path) -> Self {
        let mut learner = Self::new(path);
        learner.collaborative = true;
        learner
    }

    pub fn learn_from_error(&mut self, pattern: &str, predicted: &str, actual: &str, context: &str) {
        // Enregistrer l'erreur
        self.brain.error_history.push(ErrorRecord {
            pattern: pattern.to_string(),
            predicted_impact: predicted.to_string(),
            actual_impact: actual.to_string(),
            context: context.to_string(),
            timestamp: Utc::now().timestamp(),
        });

        // Ajuster les poids
        let stats = self.brain.patterns.entry(pattern.to_string())
            .or_insert_with(|| PatternStats {
                signature: pattern.to_string(),
                occurrences: 0,
                impact_predictions: HashMap::new(),
                success_rate: 1.0,
                last_contexts: Vec::new(),
            });

        stats.success_rate *= 0.9; // Pénaliser le pattern
        self.save_brain();
    }

    pub fn record_success(&mut self, pattern: &str, context: &str) {
        let stats = self.brain.patterns.get_mut(pattern).unwrap();
        stats.success_rate = (stats.success_rate * 0.9) + 0.1;
        stats.last_contexts.push(context.to_string());
        stats.last_contexts.truncate(10); // Garder les 10 derniers contextes

        self.brain.success_patterns.push(SuccessPattern {
            pattern: pattern.to_string(),
            context: context.to_string(),
            confidence: stats.success_rate,
        });

        self.save_brain();
    }

    pub fn predict_impact(&self, content: &str, context: &str) -> Option<String> {
        let features = self.brain.extract_features(content, context);
        self.brain.predict_with_tree(&features)
    }

    pub fn record_prediction(&mut self, content: &str, context: &str, impact: &Impact) {
        let features = self.brain.extract_features(content, context);
        if let Some(tree) = &mut self.brain.tree_learner {
            tree.add_sample(features, impact.level.clone());
        }
    }

    pub fn get_confidence(&self, prediction: &str) -> f64 {
        // Utiliser l'historique des succès et les statistiques des patterns
        if let Some(stats) = self.brain.patterns.values()
            .find(|p| p.impact_predictions.contains_key(prediction))
        {
            // Plus le pattern a été validé, plus la confiance est élevée
            let success_weight = stats.success_rate;
            let usage_weight = (stats.occurrences as f64).min(100.0) / 100.0;
            
            // Combiner les facteurs
            0.4 + (success_weight * 0.3) + (usage_weight * 0.3)
        } else {
            // Confiance minimale pour une nouvelle prédiction
            0.4
        }
    }

    pub fn analyze_change(&mut self, content: &str, context: &str) -> Impact {
        let features = self.brain.extract_features(content, context);
        
        if let Some(prediction) = self.brain.predict_with_tree(&features) {
            let confidence = self.get_confidence(&prediction);
            Impact {
                level: prediction.to_string(),  // Convertir en String
                confidence,
            }
        } else {
            // Analyse de repli basée sur les patterns
            let level = if content.contains("breaking") {
                "major"
            } else if content.contains("feat") || content.contains("feature") {
                "minor"
            } else {
                "patch"
            };

            Impact {
                level: level.to_string(),
                confidence: 0.4, // Confiance réduite pour l'analyse de repli
            }
        }
    }

    pub fn learn_from_result(&mut self, content: &str, context: &str, actual_impact: &str) {
        let features = self.brain.extract_features(content, context);
        
        if let Some(tree) = &mut self.brain.tree_learner {
            tree.add_sample(features, actual_impact.to_string());
            let _ = self.brain.train_decision_tree();
        }

        self.save_brain();
    }

    pub fn learn_from_developer(&mut self, feedback: DevFeedback) {
        // Ajuster les poids en fonction du feedback développeur
        let stats = self.brain.patterns.entry(feedback.pattern)
            .or_insert_with(|| PatternStats::default());

        if feedback.confirmed {
            // Renforcer le pattern si confirmé
            stats.success_rate = (stats.success_rate * 0.8) + 0.2;
        } else {
            // Pénaliser le pattern si corrigé
            stats.success_rate *= 0.8;
            
            // Ajouter la correction au modèle
            stats.impact_predictions
                .entry(feedback.actual_impact)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        // Partager l'apprentissage
        if let Some(global) = &mut self.global_learning {
            global.contribute(feedback);
        }
    }

    pub fn suggest_impact(&self, pattern: &str, context: &str) -> Option<String> {
        // Utiliser l'apprentissage communautaire si disponible
        if let Some(global) = &self.global_learning {
            if let Some(community_pattern) = global.shared_patterns.get(pattern) {
                if community_pattern.confirmations > 10 {
                    return Some(community_pattern.get_best_impact());
                }
            }
        }

        // Fallback sur l'apprentissage local
        self.brain.patterns.get(pattern)
            .map(|stats| stats.get_most_likely_impact())
    }

    fn save_brain(&self) {
        // Sauvegarder en binaire pour performances
        let bin_path = self.memory_path.with_extension("bin");
        if let Ok(f) = std::fs::File::create(&bin_path) {
            // Implémenter une méthode de sérialisation personnalisée si nécessaire
            // Exemple : self.brain.save_to_file(f);
            let _ = self.brain.save_to_file(&bin_path);
        }

        // Si mode collaboratif, partager l'apprentissage
        if self.collaborative {
            if let Some(global) = &self.global_learning {
                let mut global_clone = (*global).clone(); // Créer une copie mutable explicite
                let _ = global_clone.sync_with_registry(); // Utiliser la copie mutable
            }
        }
    }

    fn load_or_create_brain(path: &std::path::Path) -> VersioningBrain {
        VersioningBrain::load_from_file(path) // Utiliser load_from_file
    }
}

impl VersioningBrain {
    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.patterns)?; // Accéder directement au champ
        std::fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load_from_file(path: &std::path::Path) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(patterns) = serde_json::from_str(&content) {
                return VersioningBrain {
                    patterns,
                    ..Default::default() // Utiliser Default pour les champs restants
                };
            }
        }
        Default::default()
    }

    pub fn predict_with_tree(&self, features: &[f64]) -> Option<String> {
        self.tree_learner.as_ref().and_then(|tree| tree.predict(features))
    }
}
