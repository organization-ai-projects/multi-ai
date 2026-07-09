pub enum TrainingType {
    CodingLanguagePractice {
        source_code: String,
        target_language: String,
    },
    ProblemSolving {
        problem: String,
        goal: String,
    },
    LanguageUnderstanding {
        input_text: String,
        task: NlpTask,
    },
    MemoryRecall {
        node_id: String,
    },
    Generalization {
        examples: Vec<Example>,
    },
    SelfEvaluation {
        output: String,
        expected_quality: f32,
    },
    DialogueInteraction {
        user_prompt: String,
    },
}
