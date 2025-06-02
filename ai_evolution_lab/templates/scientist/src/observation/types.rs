use uuid::Uuid;

#[derive(Debug)]
pub enum ObservationType {
    External,    // Observation de surface (existe ? vit ?)
    Behavior,    // Observation du comportement (exécution)
    Dissection   // Analyse destructive du code source
}

#[derive(Debug)]
pub struct Observation {
    pub specimen_id: Uuid,
    pub observation_type: ObservationType,
    pub is_alive: bool,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub execution_time_ms: u64,
    pub source_code: Option<String>
}
