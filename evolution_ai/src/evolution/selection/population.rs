pub struct PopulationUtils;

impl PopulationUtils {
    /// Réduit la population à une taille cible en gardant les meilleurs individus.
    pub fn reduce_population(performers: &[(String, f64)], target_size: usize) -> Vec<String> {
        performers
            .iter()
            .take(target_size)
            .map(|(individual, _)| individual.clone())
            .collect()
    }

    /// Ajoute de nouveaux individus aléatoires pour diversifier la population.
    pub fn add_diversity<F>(population: &mut Vec<String>, num_new_individuals: usize, generator: F)
    where
        F: Fn() -> String,
    {
        for _ in 0..num_new_individuals {
            population.push(generator());
        }
    }
}
