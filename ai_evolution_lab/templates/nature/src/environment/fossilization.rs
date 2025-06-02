use crate::life_form::{LifeForm, State};

pub struct Fossilization {
    fossils: Vec<LifeForm>,
    max_fossils: usize,
}

impl Fossilization {
    pub fn new(max_fossils: usize) -> Self {
        Self {
            fossils: Vec::new(),
            max_fossils
        }
    }

    pub fn try_fossilize(&mut self, form: &mut LifeForm) {
        let total_lifetime: u64 = form.execution_results.iter()
            .map(|r| r.lifetime_ms)
            .sum();
        
        if rand::random::<f32>() < (total_lifetime as f32 / 1_000_000.0) {
            form.current_state = State::Fossilized;
            self.fossils.push(form.clone());
            
            while self.fossils.len() > self.max_fossils {
                self.fossils.remove(0);
            }
        }
    }
}
