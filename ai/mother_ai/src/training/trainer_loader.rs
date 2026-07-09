use crate::training::training_types::{Trainer, TrainingKind};
use crate::training::code::rust::rust_trainer::RustTrainer;
use crate::training::code::php::php_trainer::PhpTrainer;
use crate::training::nlp::nlp_trainer::NlpTrainer;
use crate::training::audio::audio_trainer::AudioTrainer;

pub fn load_trainer(kind: &TrainingKind) -> Box<dyn Trainer> {
    match kind {
        TrainingKind::Rust => Box::new(RustTrainer),
        TrainingKind::PHP => Box::new(PhpTrainer),
        TrainingKind::NLP => Box::new(NlpTrainer),
        TrainingKind::Audio => Box::new(AudioTrainer),
    }
}
