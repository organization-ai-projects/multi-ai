pub struct LocalCatastrophe;

impl LocalCatastrophe {
    pub fn trigger<F>(&self, population: &mut Vec<String>, filter: F)
    where
        F: Fn(&String) -> bool,
    {
        population.retain(|individual| !filter(individual));
    }
}
