use serde::{Serialize, Deserialize};
use crate::execution::ExecutionResult;

#[derive(Serialize, Deserialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct LifeForm {
    pub id: String,
    pub birth_time: i64,
    pub last_seen: i64,
    pub total_lifetime: u64,
    pub current_state: State,
    pub source_code: String,
    pub execution_results: Vec<ExecutionResult>,
    pub parent_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum State {
    Alive,
    Dead,
    Fossilized, // Mort mais traces conservées
}

impl LifeForm {
    pub fn new(id: String, source_code: String, parent_id: Option<String>) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            birth_time: now,
            last_seen: now,
            total_lifetime: 0,
            current_state: State::Alive,
            source_code,
            execution_results: Vec::new(),
            parent_id,
        }
    }

    /// Met à jour l'état en fonction du résultat d'exécution
    pub fn update_state(&mut self, result: ExecutionResult) {
        self.last_seen = chrono::Utc::now().timestamp();
        self.total_lifetime += result.lifetime_ms;
        self.execution_results.push(result.clone());

        if !result.survived {
            self.current_state = State::Dead;
        }
    }
}
