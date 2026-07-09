//Tu ajoutes dans Primitive::Custom la possibilité de lancer un binaire Rust ou une lib dynamique, ou même d’interpréter une nouvelle IA pipeline.
//Ou tu peux charger dynamiquement des libs (via libloading), ou faire du WASM plug si tu veux aller plus loin plus tard…
use std::process::Command;

impl Primitive {
    pub fn execute_external(&self, input: f64) -> Option<f64> {
        match self {
            Primitive::Custom(code_path) => {
                // Exécution d’un binaire externe (ex: "./agents/myagent")
                let output = Command::new(code_path)
                    .arg(input.to_string())
                    .output()
                    .ok()?;
                let out_str = String::from_utf8_lossy(&output.stdout);
                out_str.trim().parse::<f64>().ok()
            }
            _ => None,
        }
    }
}
