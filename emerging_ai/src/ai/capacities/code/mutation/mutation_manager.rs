use super::mutation_types::{MutationIntensity, MutationResult, MutationType};
use crate::ai::brain::observable::{AutoObservable, Observable};
use rand::{seq::SliceRandom, rng, Rng};
use syn::{parse_str, ItemFn};

pub struct MutationManager {
    last_mutations: Vec<MutationResult>,
}

impl MutationManager {
    pub fn new() -> Self {
        Self {
            last_mutations: Vec::new(),
        }
    }

    // Une seule méthode publique simple
    pub fn apply_mutation(&mut self, code: &str, strategy: &str) -> Option<String> {
        let mutation_type = match strategy {
            "shuffle" => MutationType::Shuffle, // Réorganise le code
            "rename" => MutationType::Rename,   // Renomme les variables
            "invert" => MutationType::Invert,   // Inverse la logique
            "random" => MutationType::Random,   // Mutation aléatoire
            _ => return None,
        };

        let result = self.execute_mutation(code, mutation_type);

        // L'observation est déléguée au code_manager
        result.map(|m| m.mutated_code)
    }

    // Une seule méthode privée qui gère toutes les mutations
    fn execute_mutation(&self, code: &str, mutation_type: MutationType) -> Option<MutationResult> {
        match mutation_type {
            MutationType::Shuffle => self.shuffle_code(code),
            MutationType::Rename => self.rename_elements(code),
            MutationType::Invert => self.invert_logic(code),
            MutationType::Random => self.random_mutation(code),
        }
    }

    // Implémentations des mutations spécifiques...
    fn shuffle_code(&self, code: &str) -> Option<MutationResult> {
        let ast: ItemFn = parse_str(code).ok()?;
        let mut stmts = ast.block.stmts.clone();
        stmts.shuffle(&mut rng());

        let mut new_fn = ast.clone();
        new_fn.block.stmts = stmts;

        Some(MutationResult {
            original_code: code.to_string(),
            mutated_code: quote!(#new_fn).to_string(),
            mutation_type: MutationType::Shuffle,
            success_score: None,
        })
    }

    fn rename_elements(&self, code: &str) -> Option<MutationResult> {
        let mut ast: ItemFn = parse_str(code).ok()?;
        let mut rng = rng();

        // Mutation pure : renommage complètement aléatoire
        for stmt in &mut ast.block.stmts {
            if let syn::Stmt::Local(local) = stmt {
                if let syn::Pat::Ident(ref mut pat_ident) = *local.pat {
                    // Génère un nom totalement aléatoire
                    let chars: String = (0..rng.random_range(1..20))
                        .map(|_| rng.random_range(b'a'..=b'z') as char)
                        .collect();

                    pat_ident.ident = syn::Ident::new(&chars, pat_ident.ident.span());

                    return Some(MutationResult {
                        original_code: code.to_string(),
                        mutated_code: quote!(#ast).to_string(),
                        mutation_type: MutationType::Rename,
                        success_score: None,
                    });
                }
            }
        }
        None
    }

    fn invert_logic(&self, code: &str) -> Option<MutationResult> {
        let mut ast: ItemFn = parse_str(code).ok()?;
        let mut rng = rng();

        // Mutations aléatoires sur la logique
        for stmt in &mut ast.block.stmts {
            if rng.random_bool(0.5) {
                // Chance aléatoire de muter
                match stmt {
                    // Mutation d'une condition : on insère des opérateurs aléatoires
                    syn::Stmt::Expr(syn::Expr::If(expr_if)) => {
                        let operators = ["!", "||", "&&"];
                        let op = operators.choose(&mut rng)?;
                        let mutated = format!("{} {}", op, quote!(#expr_if.cond));
                        expr_if.cond = Box::new(parse_str(&mutated).ok()?);
                    }
                    // Autres mutations possibles sur d'autres types de statements
                    _ => continue,
                }
            }
        }

        Some(MutationResult {
            original_code: code.to_string(),
            mutated_code: quote!(#ast).to_string(),
            mutation_type: MutationType::Invert,
            success_score: None,
        })
    }

    fn random_mutation(&self, code: &str) -> Option<MutationResult> {
        let strategies = vec![
            MutationType::Shuffle,
            MutationType::Rename,
            MutationType::Invert,
            MutationType::Random,
        ];

        if let Some(strategy) = strategies.choose(&mut rng()) {
            match strategy {
                MutationType::Shuffle => self.shuffle_code(code),
                MutationType::Rename => self.rename_elements(code),
                MutationType::Invert => self.invert_logic(code),
                MutationType::Random => self.random_mutation(code),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn apply_random_mutation(
        &mut self,
        code: &str,
        intensity: MutationIntensity,
    ) -> Option<String> {
        let ast = parse_str(code).ok()?;
        let mut rng = rng();

        match intensity {
            MutationIntensity::Micro => {
                // Mutation au niveau caractère/token
                self.mutate_chars(&ast, rng.random_range(1..5))
            }
            MutationIntensity::Minor => {
                // Mutation au niveau expression
                self.mutate_expressions(&ast, rng.random_range(1..3))
            }
            MutationIntensity::Major => {
                // Mutation au niveau bloc
                self.mutate_blocks(&ast, rng.random_range(1..2))
            }
            MutationIntensity::Radical => {
                // Mutation complète
                self.mutate_entire_code(&ast)
            }
        }
    }
}

impl Observable for MutationResult {
    fn get_signature(&self) -> String {
        format!(
            "mutation:{}:{}",
            self.mutation_type,
            self.success_score.unwrap_or(0.0)
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AutoObservable for MutationResult {}
