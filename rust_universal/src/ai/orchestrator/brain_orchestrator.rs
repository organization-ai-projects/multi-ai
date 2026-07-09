use crate::ai::dialogue::generator::process_dialogue;
use crate::ai::dialogue::respond_to_input;
use crate::ai::decision::action::decide_action;
use crate::ai::motivation::evaluate::evaluate_goals;
use crate::ai::perception::parser::run_perception;
use crate::ai::cognition::planner::run_cognition;
use crate::ai::learning::learner::run_learning;

pub struct BrainCycleConfig {
    pub enable_perception: bool,
    pub enable_cognition: bool,
    pub enable_learning: bool,
    pub enable_motivation: bool,
    pub enable_decision: bool,
    pub enable_dialogue: bool, // Nouvelle option pour la phase de dialogue
}

impl Default for BrainCycleConfig {
    fn default() -> Self {
        Self {
            enable_perception: true,
            enable_cognition: true,
            enable_learning: true,
            enable_motivation: true,
            enable_decision: true,
            enable_dialogue: true, // Activée par défaut
        }
    }
}

pub fn run_brain_cycle_with_config(config: BrainCycleConfig, dialogue_input: &str) {
    println!("🔄 Initialisation...");

    if config.enable_perception {
        println!("🔍 Phase de perception...");
        run_perception("Je suis une IA.");
    }

    if config.enable_cognition {
        println!("🧠 Phase de cognition...");
        run_cognition();
    }

    if config.enable_learning {
        println!("📚 Phase d'apprentissage...");
        run_learning();
    }

    if config.enable_motivation {
        println!("🎯 Phase de motivation...");
        evaluate_goals();
    }

    if config.enable_decision {
        println!("🤔 Phase de décision...");
        let action = decide_action();
        println!("{}", action);
    }

    if config.enable_dialogue {
        println!("💬 Phase de dialogue...");
        process_dialogue(dialogue_input);
    }
}
