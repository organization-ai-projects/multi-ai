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

    pub fn get_form_mut(&mut self, id: &str) -> Option<&mut LifeForm> {
        self.forms.get_mut(id)
    }

    pub fn get_form(&self, id: &str) -> Option<&LifeForm> {
        self.forms.get(id)
    }

    pub fn get_dead_forms(&self) -> impl Iterator<Item = &LifeForm> {
        self.forms.values().filter(|f| matches!(f.current_state, State::Dead))
    }

    pub fn get_all_forms(&self) -> impl Iterator<Item = &LifeForm> {
        self.forms.values()
    }

    pub fn count_living(&self) -> usize {
        self.get_living_forms().count()
    }

    pub fn count_dead(&self) -> usize {
        self.get_dead_forms().count()
    }

    pub fn total_lifetime(&self) -> u64 {
        self.forms.values()
            .map(|form| form.total_lifetime)
            .sum()
    }

    pub fn add_form(&mut self, form: LifeForm) {
        self.forms.insert(form.id.clone(), form);
    }

    pub fn remove_form(&mut self, id: &str) {
        self.forms.remove(id);
    }
}
