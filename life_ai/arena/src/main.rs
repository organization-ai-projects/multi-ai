use std::fs;
use std::path::Path;
use wasmtime::*;

const AGENTS_PATH: &str = "./agents/";
const N_AGENTS: usize = 5;
const TICKS: usize = 100;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let linker = Linker::new(&engine);

    // Charge tous les agents wasm du dossier agents
    let mut agents: Vec<Module> = vec![];
    for n in 1..=N_AGENTS {
        let path = format!(
            "{}/agent{}/target/wasm32-unknown-unknown/release/agent{}.wasm",
            AGENTS_PATH, n, n
        );
        if Path::new(&path).exists() {
            agents.push(Module::from_file(&engine, &path)?);
        } else {
            eprintln!("Agent {} non trouvé, pense à compiler tes agents.", n);
        }
    }

    let mut store = Store::new(&engine, ());
    let mut agent_states = vec![0u64; agents.len()]; // Ex: score ou "état interne"

    // Main loop
    for tick in 0..TICKS {
        for (i, agent_module) in agents.iter().enumerate() {
            let instance = linker.instantiate(&mut store, agent_module)?;
            // Convention: chaque agent expose une fonction "act" (entrée = défi simple, sortie = action ou score)
            let act = instance.get_typed_func::<u64, u64, _>(&mut store, "act")?;
            let input = tick as u64; // ex: défi simple (le tick du tour)
            let output = act.call(&mut store, input)?;
            agent_states[i] = output; // Stocke le retour (score, décision, ...)
            println!("Agent {} a répondu: {} (tick {})", i + 1, output, tick);
        }
        // Tu peux ici ajouter la logique d'entraide, compétition, mutation (modification binaire, recompilation, swap agent, etc.)
    }

    Ok(())
}
