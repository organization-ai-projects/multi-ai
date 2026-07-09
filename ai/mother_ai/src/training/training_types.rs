#[derive(Debug, Clone)]
pub enum TrainingKind {
    Rust,
    PHP,
    NLP,
    Audio,
}

pub trait Trainer {
    fn train(&self, data: &str);
}
