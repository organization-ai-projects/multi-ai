use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array2, Array1};
use serde::{Serialize, Deserialize};

#[derive(Debug)] // Supprimer Serialize et Deserialize
pub struct DecisionTreeLearner {
    features: Vec<Vec<f64>>,
    labels: Vec<String>,
    tree: Option<DecisionTree<f64, String>>,
}

impl DecisionTreeLearner {
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
            labels: Vec::new(),
            tree: None,
        }
    }

    pub fn add_sample(&mut self, features: Vec<f64>, impact: String) {
        self.features.push(features);
        self.labels.push(impact);
    }

    pub fn train(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.features.is_empty() {
            return Ok(());
        }

        let x = Array2::from_shape_vec(
            (self.features.len(), self.features[0].len()),
            self.features.iter().flatten().cloned().collect(),
        )?;
        let y = Array1::from_vec(self.labels.clone());

        let dataset = Dataset::new(x, y);
        self.tree = Some(
            DecisionTree::params()
                .max_depth(Some(5))
                .fit(&dataset)?,
        );

        Ok(())
    }

    pub fn predict(&self, features: &[f64]) -> Option<String> {
        self.tree.as_ref().map(|tree| {
            let x = Array2::from_shape_vec(
                (1, features.len()),
                features.to_vec()
            ).unwrap();
            tree.predict(&x)[0].clone()
        })
    }

    pub fn get_confidence(&self, prediction: &str) -> f64 {
        // Calculer la confiance basée sur la fréquence de la prédiction
        let total_samples = self.labels.len() as f64;
        if total_samples == 0.0 {
            return 0.5;
        }

        let matching_samples = self.labels.iter()
            .filter(|&label| label == prediction)
            .count() as f64;

        matching_samples / total_samples
    }
}

// Ajouter ndarray au Cargo.toml
// [dependencies]
// ndarray = "0.15"
