use crate::environment::{Population, FileSystem};
use crate::life_form::LifeForm;

pub struct LifeManager;

impl LifeManager {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_reproduction(&self, population: &mut Population, fs: &FileSystem) {
        let new_forms = population.get_living_forms()
            .map(|form| {
                let new_id = format!("{}_child", form.id);
                LifeForm::new(new_id, form.source_code.clone(), Some(form.id.clone()))
            })
            .collect::<Vec<_>>();

        for form in new_forms {
            fs.save_life_form(&form);
            population.add_form(form);
        }
    }

    pub fn remove_dead_forms(&self, population: &mut Population) {
        let dead_ids = population.get_dead_forms()
            .map(|form| form.id.clone())
            .collect::<Vec<_>>();

        for id in dead_ids {
            population.remove_form(&id);
        }
    }
}
