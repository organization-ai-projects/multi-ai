pub enum NlpTask {
    Translate { to_lang: String },
    Summarize,
    ExtractFacts,
    Reformulate,
}