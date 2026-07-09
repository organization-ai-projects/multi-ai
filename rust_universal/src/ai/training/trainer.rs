use crate::memory::graph::MemoryGraph;

use crate::training::TrainingTask;
use crate::training::code::coding_language_practice::CodingLanguagePractice;
use crate::training::logic::problem_solving::ProblemSolving;
use crate::training::self_monitoring::self_evaluation::SelfEvaluation;

pub struct Trainer;

impl Trainer {
    pub fn new() -> Self {
        Trainer
    }

    pub fn execute(&self, task: TrainingTask, memory: &mut MemoryGraph) {
        match task {
            TrainingTask::CodingLanguagePractice(t) => t.run(memory),
            TrainingTask::ProblemSolving(t) => t.run(memory),
            TrainingTask::SelfEvaluation(t) => t.run(memory),
        }
    }
}
