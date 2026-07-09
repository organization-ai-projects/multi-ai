use crate::ai::brain::Brain;

pub struct BrainMotivation;

impl BrainMotivation {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self) {
        println!("Motivation évaluée.");
    }
}
