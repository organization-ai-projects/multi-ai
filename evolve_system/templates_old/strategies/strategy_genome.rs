#[derive(Clone, Debug)]
pub struct StrategyGenome {
    pub ops: Vec<Op>,
    pub fitness: f64,
    pub tries: usize,
    pub successes: usize,
}

impl StrategyGenome {
    pub fn new_random(rng: &mut impl Rng) -> Self {
        let len = rng.gen_range(1..=5);
        let ops = (0..len)
            .map(|_| Op::all().choose(rng).unwrap().clone())
            .collect();
        StrategyGenome {
            ops,
            fitness: 0.0,
            tries: 0,
            successes: 0,
        }
    }

    pub fn apply(&self, input: &str) -> String {
        let mut out = input.to_string();
        for op in &self.ops {
            out = match op {
                Op::Upper => out.to_uppercase(),
                Op::Lower => out.to_lowercase(),
                Op::Reverse => out.chars().rev().collect(),
                Op::Double => format!("{}{}", out, out),
                Op::Exclamation => format!("{}!", out),
                Op::Replace(a, b) => out.replace(*a, &b.to_string()),
            }
        }
        out
    }

    /// Mutation : remplace, insère ou supprime une opération
    pub fn mutate(&mut self, rng: &mut impl Rng) {
        if self.ops.is_empty() || rng.gen_bool(0.3) {
            // Ajouter une op
            self.ops.insert(
                rng.gen_range(0..=self.ops.len()),
                Op::all().choose(rng).unwrap().clone(),
            );
        }
        if !self.ops.is_empty() && rng.gen_bool(0.3) {
            // Modifier une op existante
            self.ops[rng.gen_range(0..self.ops.len())] = Op::all().choose(rng).unwrap().clone();
        }
        if self.ops.len() > 1 && rng.gen_bool(0.2) {
            // Supprimer une op
            self.ops.remove(rng.gen_range(0..self.ops.len()));
        }
    }

    /// Crossover (mélange de 2 génomes)
    pub fn crossover(a: &StrategyGenome, b: &StrategyGenome, rng: &mut impl Rng) -> StrategyGenome {
        let mut ops = Vec::new();
        let alen = a.ops.len();
        let blen = b.ops.len();
        let len = alen.max(blen);
        for i in 0..len {
            if rng.gen_bool(0.5) {
                if i < alen {
                    ops.push(a.ops[i].clone());
                }
            } else {
                if i < blen {
                    ops.push(b.ops[i].clone());
                }
            }
        }
        StrategyGenome {
            ops,
            fitness: 0.0,
            tries: 0,
            successes: 0,
        }
    }
}
