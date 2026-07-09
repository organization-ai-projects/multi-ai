use rand::seq::SliceRandom;

type StratFn = Box<dyn Fn(&str) -> String + Send + Sync>;

pub struct Explorer {
    pub history: Vec<Experiment>,
    pub strategies: Vec<StratFn>,
}

impl Explorer {
    pub fn meta_explore(&mut self) {
        let mut rng = rand::thread_rng();

        if self.strategies.len() < 2 {
            return;
        }

        // Tire 2 stratégies au hasard
        let (f1, f2) = {
            let mut choices = self.strategies.choose_multiple(&mut rng, 2);
            (choices.next().unwrap(), choices.next().unwrap())
        };

        // Crée une nouvelle stratégie : composition (f2(f1(input)))
        let new_strat: StratFn = Box::new(move |input| {
            let mid = f1(input);
            f2(&mid)
        });

        // (Optionnel) Donne-lui un nom, un id, etc.
        // Stocke la stratégie dans la liste
        self.strategies.push(new_strat);
    }
}
