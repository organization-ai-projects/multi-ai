use std::collections::HashMap; // Ajouter cet import
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]  // Ajouter Clone
pub struct DevFeedback {
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub pattern: String,
    pub context: String,
    pub suggested_impact: String,
    pub actual_impact: String,
    pub confirmed: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Ajouter Clone
pub struct CommunityPattern {
    pub confirmations: u32,
    pub corrections: HashMap<String, u32>,
    pub contexts: HashMap<String, u32>,
}

impl CommunityPattern {
    pub fn get_best_impact(&self) -> String {
        self.corrections
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(impact, _)| impact.clone())
            .unwrap_or_else(|| "patch".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Ajouter Clone
pub struct GlobalLearning {
    pub feedbacks: Vec<DevFeedback>,
    pub shared_patterns: HashMap<String, CommunityPattern>,
    pub last_sync: DateTime<Utc>,
}

impl GlobalLearning {
    pub fn contribute(&mut self, feedback: DevFeedback) {
        // Ajouter le feedback local
        self.feedbacks.push(feedback.clone());

        // Mettre à jour les patterns communautaires
        let pattern = self.shared_patterns
            .entry(feedback.pattern.clone())
            .or_insert_with(|| CommunityPattern {
                confirmations: 0,
                corrections: HashMap::new(),
                contexts: HashMap::new(),
            });

        if feedback.confirmed {
            pattern.confirmations += 1;
        } else {
            *pattern.corrections
                .entry(feedback.actual_impact)
                .or_insert(0) += 1;
        }

        *pattern.contexts
            .entry(feedback.context)
            .or_insert(0) += 1;

        // Sauvegarder
        self.save();
    }

    pub fn sync_with_registry(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implémenter la synchronisation avec un registre central
        // Par exemple via un serveur ou GitHub
        Ok(())
    }

    fn save(&mut self) {
        // Implémenter la sauvegarde
        // Par exemple :
        if let Ok(json) = serde_json::to_string(self) {
            let _ = std::fs::write("global_learning.json", json);
        }
    }
}
