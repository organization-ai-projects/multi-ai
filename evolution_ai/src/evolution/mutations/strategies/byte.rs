use super::super::types::{MutationProvider, MutationResult};
use rand::Rng;

#[derive(Clone)]
pub struct ByteMutator {
    mutation_rate: f64,
}

impl ByteMutator {
    pub fn new(rate: f64) -> Self {
        Self {
            mutation_rate: rate,
        }
    }
}

impl MutationProvider for ByteMutator {
    fn mutate(&self, code: &str, max_mutations: usize) -> MutationResult {
        let mut rng = rand::thread_rng();
        let mut bytes = code.as_bytes().to_vec();
        let original = bytes.clone(); // Sauvegarde de l'état initial
        let mut mutations_applied = 0;

        while mutations_applied < max_mutations && rng.gen_bool(self.mutation_rate) {
            let idx = rng.gen_range(0..bytes.len());
            let original_byte = bytes[idx];

            bytes[idx] = match original_byte {
                b'0'..=b'9' => rng.gen_range(b'0'..=b'9'),
                b'a'..=b'z' => rng.gen_range(b'a'..=b'z'),
                b'A'..=b'Z' => rng.gen_range(b'A'..=b'Z'),
                b'+' | b'-' | b'*' | b'/' => [b'+', b'-', b'*', b'/'][rng.gen_range(0..4)],
                _ => continue,
            };

            // Vérifier l'UTF-8
            if std::str::from_utf8(&bytes).is_err() {
                bytes[idx] = original_byte; // Restaurer l'octet original en cas d'erreur
                continue;
            }

            mutations_applied += 1;
        }

        // Si aucune mutation valide n'a été appliquée, retourner l'état initial
        let code = if mutations_applied > 0 {
            String::from_utf8(bytes).unwrap_or_else(|_| code.to_string())
        } else {
            String::from_utf8(original).unwrap_or_else(|_| code.to_string())
        };

        MutationResult {
            code,
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

// Déplacement réel du contenu de `mutations/byte.rs`.
