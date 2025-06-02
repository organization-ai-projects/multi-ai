use std::collections::HashMap;
use crate::life_form::{LifeForm, State};
use crate::execution::ExecutionResult;
use uuid::uuid7;

pub struct Population {
    forms: HashMap<String, LifeForm>,
}

impl Population {
    pub fn new() -> Self {
        Self {
            forms: HashMap::new()
        }
    }

    pub fn track_life_form(&mut self, source_code: String, parent: Option<String>) -> String {
        let id = uuid7().to_string();
        let form = LifeForm::new(id.clone(), source_code, parent);
        self.forms.insert(id.clone(), form);
        id
    }

    pub fn process_results(&mut self, id: &str, result: ExecutionResult) {
        if let Some(form) = self.forms.get_mut(id) {
            form.update_state(result);
        }
    }

    pub fn get_living_forms(&self) -> impl Iterator<Item = &LifeForm> {
        self.forms.values().filter(|f| matches!(f.current_state, State::Alive))
    }
}
