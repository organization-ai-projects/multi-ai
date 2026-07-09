pub struct MetaExplorer {
    pub population: Vec<StrategyGenome>,
    pub rng: rand::rngs::ThreadRng,
}

impl MetaExplorer {
    pub fn new(n: usize) -> Self {
        let mut rng = rand::rng();
        Self {
            population: (0..n)
                .map(|_| StrategyGenome::new_random(&mut rng))
                .collect(),
            rng,
        }
    }

    /// Teste la population sur un batch d'inputs et objectifs
    pub fn evaluate_population(&mut self, inputs: &[(&str, &str)]) {
        for strat in &mut self.population {
            strat.tries += inputs.len();
            let mut score = 0.0;
            let mut successes = 0;
            for (input, target) in inputs {
                let out = strat.apply(input);
                if &out == target {
                    successes += 1;
                }
                // Fitness : bonus pour ressemblance, ou ce que tu veux
                score += jaccard_similarity(&out, target);
            }
            strat.fitness = score / inputs.len() as f64;
            strat.successes += successes;
        }
    }

    /// Sélection naturelle : garde les meilleurs (fitness)
    pub fn natural_selection(&mut self, top_n: usize) {
        self.population
            .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        self.population.truncate(top_n);
    }

    /// Reproduction (mutation/crossover pour retrouver une population cible)
    pub fn reproduce(&mut self, pop_size: usize) {
        let mut new_pop = self.population.clone();
        while new_pop.len() < pop_size {
            let a = new_pop.choose(&mut self.rng).unwrap();
            let b = new_pop.choose(&mut self.rng).unwrap();
            let mut child = StrategyGenome::crossover(a, b, &mut self.rng);
            child.mutate(&mut self.rng);
            new_pop.push(child);
        }
        self.population = new_pop;
    }
}

// Simplicité : similarité naïve
fn jaccard_similarity(a: &str, b: &str) -> f64 {
    let aset: std::collections::HashSet<_> = a.chars().collect();
    let bset: std::collections::HashSet<_> = b.chars().collect();
    let inter = aset.intersection(&bset).count() as f64;
    let union = aset.union(&bset).count() as f64;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}
