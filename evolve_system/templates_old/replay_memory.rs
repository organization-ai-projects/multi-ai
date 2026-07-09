use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryEvent {
    pub generation: usize,
    pub timestamp: DateTime<Utc>,
    pub best_strategy: Strategy,
    pub population_snapshot: Vec<Strategy>,
    pub fitness_snapshot: Vec<f64>,
}

// Enregistre à chaque génération :
pub fn log_generation(generation: usize, population: &Population) -> std::io::Result<()> {
    let best = population
        .individuals
        .iter()
        .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap();
    let event = MemoryEvent {
        generation,
        timestamp: Utc::now(),
        best_strategy: best.clone(),
        population_snapshot: population.individuals.clone(),
        fitness_snapshot: population.individuals.iter().map(|s| s.fitness).collect(),
    };
    let path = format!("history/gen_{:05}.ron", generation);
    let pretty = ron::ser::PrettyConfig::default();
    let ron_str = ron::ser::to_string_pretty(&event, pretty).unwrap();
    std::fs::write(path, ron_str)?;
    Ok(())
}

// Reload old generations, analyse, revive, etc.
pub fn load_generation(path: &str) -> MemoryEvent {
    let ron_str = std::fs::read_to_string(path).unwrap();
    ron::de::from_str(&ron_str).unwrap()
}
