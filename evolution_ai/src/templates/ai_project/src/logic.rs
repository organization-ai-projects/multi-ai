use crate::memory::Memory;

pub fn process(input: &[f64], memory: &mut Memory) -> Vec<f64> {
    // Ex : logique simple, parfait pour muter automatiquement
    if memory.last_reward > 0.0 {
        input.iter().map(|x| x + 1.0).collect()
    } else {
        input.iter().map(|x| x * -1.0).collect()
    }
}
