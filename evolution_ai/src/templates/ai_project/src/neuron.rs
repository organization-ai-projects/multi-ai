use crate::memory::Memory;

// Dummy neural net, à remplacer par une vraie mutée si besoin !
pub fn process(input: &[f64], memory: &mut Memory) -> Vec<f64> {
    // Ex : somme pondérée des entrées (poids stockés en mémoire)
    let weights = if memory.data.is_empty() {
        vec![1.0; input.len()]
    } else {
        memory.data.clone()
    };
    input
        .iter()
        .zip(weights.iter())
        .map(|(x, w)| x * w)
        .collect()
}
