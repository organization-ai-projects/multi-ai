use clap::Parser;
use reqwest::blocking::multipart;
use reqwest::blocking::Client;
use std::fs;

#[derive(Parser)]
struct Args {
    /// Nom du projet .unl (sans extension)
    #[arg(short, long)]
    name: String,

    /// Fichier à uploader
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let url = "http://localhost:8080/upload";
    let file_bytes = fs::read(&args.file).expect("Fichier introuvable");

    let part = multipart::Part::bytes(file_bytes).file_name(format!("{}.unl", args.name));
    let form = multipart::Form::new().part("file", part);

    let client = Client::new();
    let res = client.post(url).multipart(form).send().unwrap();

    if res.status().is_success() {
        println!("✅ Projet '{}' publié avec succès", args.name);
    } else {
        println!("❌ Échec de publication : {}", res.status());
    }
}
