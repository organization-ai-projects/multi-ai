pub struct GlobalCatastrophe;

impl GlobalCatastrophe {
    pub fn trigger(&self, population: &mut Vec<String>, survival_rate: f64) {
        let mut rng = rand::rng();
        population.retain(|_| rng.random_bool(survival_rate));
    }
}
