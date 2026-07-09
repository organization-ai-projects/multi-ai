use rand::Rng;

#[derive(Clone)]
pub enum SelectionMethod {
    Roulette,
    Tournament(usize), // Taille du tournoi
}

#[derive(Clone)]
pub struct SelectionStrategy {
    pub selection_pressure: f64,
    pub min_population: usize,
    pub method: SelectionMethod,
}

impl SelectionStrategy {
    pub fn new(selection_pressure: f64, min_population: usize, method: SelectionMethod) -> Self {
        Self {
            selection_pressure,
            min_population,
            method,
        }
    }

    pub fn select_parents(&self, performers: &[(String, f64)]) -> Option<(String, String)> {
        match &self.method {
            SelectionMethod::Roulette => self.select_parents_roulette(performers),
            SelectionMethod::Tournament(size) => self.select_parents_tournament(performers, *size),
        }
    }

    fn select_parents_roulette(&self, performers: &[(String, f64)]) -> Option<(String, String)> {
        if performers.len() < 2 {
            return None;
        }

        let total_fitness: f64 = performers.iter().map(|(_, score)| score.max(0.0)).sum();
        if total_fitness <= 0.0 {
            return None;
        }

        let mut rng = rand::rng();
        let mut select_one = |exclude: Option<&String>| {
            let mut sum = 0.0;
            let target = rng.gen::<f64>() * total_fitness;

            for (path, score) in performers {
                if exclude.map_or(true, |ex| path != ex) {
                    sum += score.max(0.0);
                    if sum >= target {
                        return Some(path.clone());
                    }
                }
            }
            None
        };

        Some((select_one(None)?, select_one(Some(&performers[0].0))?))
    }

    fn select_parents_tournament(
        &self,
        performers: &[(String, f64)],
        tournament_size: usize,
    ) -> Option<(String, String)> {
        if performers.len() < 2 {
            return None;
        }

        let mut rng = rand::rng();
        let mut select_one = || {
            let mut best: Option<(String, f64)> = None;
            for _ in 0..tournament_size {
                let candidate = &performers[rng.random_range(0..performers.len())];
                if best.is_none() || candidate.1 > best.as_ref().unwrap().1 {
                    best = Some(candidate.clone());
                }
            }
            best.map(|(path, _)| path)
        };

        Some((select_one()?, select_one()?))
    }

    /// Évalue un individu et retourne un score de performance.
    pub fn evaluate_individual(&self, individual: &str) -> f64 {
        // Exemple : la performance est basée sur la longueur du code et un facteur de pression.
        let base_score = individual.len() as f64;
        base_score * self.selection_pressure
    }
}
