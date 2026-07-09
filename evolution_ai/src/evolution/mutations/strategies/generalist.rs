use super::super::types::{MutationProvider, MutationResult};
use rand::{prelude::SliceRandom, Rng};

#[derive(Clone)]
pub struct GeneralistMutator {
    mutation_rate: f64,
    patterns: Vec<(&'static str, &'static str)>,
}

impl GeneralistMutator {
    pub fn new(rate: f64) -> Self {
        Self {
            mutation_rate: rate,
            patterns: vec![
                // Mutations génériques
                ("fn", "pub fn"),
                ("let", "let mut"),
                ("Vec<", "Box<"),
                ("&str", "String"),
                ("unwrap()", "expect(\"ok\")"),
                ("if ", "while "),
                ("return", "continue"),
                ("> 0", ">= 0"),
                ("+ 1", "* 2"),
                ("String::", "str::"),
                (".clone()", ".to_owned()"),
                (".collect()", ".collect::<Vec<_>>()"),
                ("None", "Some(Default::default())"),
            ],
        }
    }
}

impl MutationProvider for GeneralistMutator {
    fn mutate(&self, code: &str, max_mutations: usize) -> MutationResult {
        let mut rng = rand::thread_rng();
        let mut mutated = code.to_string();
        let mut mutations_applied = 0;

        // Utiliser un enum pour représenter les stratégies
        enum Strategy {
            Pattern,
            Insertion,
            Expression,
        }

        while mutations_applied < max_mutations && rng.gen_bool(self.mutation_rate) {
            let old_code = mutated.clone();

            // Choisir une stratégie
            let strategy = match rng.gen_range(0..10) {
                0..=3 => Strategy::Pattern,   // 40%
                4..=5 => Strategy::Insertion, // 20%
                _ => Strategy::Expression,    // 40%
            };

            mutated = match strategy {
                Strategy::Pattern => {
                    if let Some((pattern, replacement)) = self.patterns.choose(&mut rng) {
                        mutated.replacen(pattern, replacement, 1)
                    } else {
                        mutated
                    }
                }
                Strategy::Insertion => {
                    let insertions = [
                        "debug_assert!(true);\n    ",
                        "#[inline]\n    ",
                        "let _guard = std::sync::Arc::new(());\n    ",
                    ];
                    let mut lines: Vec<_> = mutated.lines().collect();
                    if !lines.is_empty() {
                        let pos = rng.gen_range(0..lines.len());
                        lines.insert(pos, insertions.choose(&mut rng).unwrap());
                        lines.join("\n")
                    } else {
                        mutated
                    }
                }
                Strategy::Expression => {
                    let expressions = [
                        ("+", "*"),
                        ("==", "!="),
                        ("Some", "None"),
                        ("true", "false"),
                        ("0", "1"),
                    ];
                    if let Some((from, to)) = expressions.choose(&mut rng) {
                        mutated.replacen(from, to, 1)
                    } else {
                        mutated
                    }
                }
            };

            if mutated != old_code {
                mutations_applied += 1;
            }
        }

        MutationResult {
            code: mutated,
            success: mutations_applied > 0,
            mutations_applied,
        }
    }

    fn get_mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    fn set_mutation_rate(&mut self, rate: f64) {
        self.mutation_rate = rate.clamp(0.0, 1.0);
    }

    fn clone_box(&self) -> Box<dyn MutationProvider> {
        Box::new(self.clone())
    }
}
