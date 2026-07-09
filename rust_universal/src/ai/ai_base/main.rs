// ai_base/src/main.rs
use std::fs;
use ron::de::from_str;

#[derive(serde::Deserialize)]
struct Metadata {
    id: String,
    name: String,
}

fn load_metadata() -> Metadata {
    let content = fs::read_to_string("metadata.ron").expect("metadata.ron missing");
    from_str(&content).expect("Invalid metadata format")
}

fn main() {
    let metadata = load_metadata();
    println!("🚀 Launching IA: {}", metadata.id);
    // Ici ta boucle principale d’IA
}
