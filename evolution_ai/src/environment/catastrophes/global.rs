pub struct GlobalCatastrophe;

impl GlobalCatastrophe {
    pub fn trigger(&self, population: &mut Vec<String>, survival_rate: f64) {
        let mut rng = rand::thread_rng();
        population.retain(|_| rng.gen_bool(survival_rate));
    }
}
