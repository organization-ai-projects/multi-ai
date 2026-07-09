use crate::engine::runtime::Runtime;
use crate::utils::sanitize_input;
use unilang::{lexer, parser};

pub fn navigate_to_url(runtime: &mut Runtime, url: &str) {
    // Sanitiser l'URL complète avant de la traiter
    let sanitized_url = sanitize_input(url);
    println!("🔄 Navigation vers: {}", sanitized_url);

    if let Some(domain) = sanitized_url.strip_prefix("unl://") {
        // Sanitiser également le domaine extrait pour plus de sécurité
        let sanitized_domain = sanitize_input(domain);

        // Si c'est la page d'accueil du navigateur
        if sanitized_domain.to_lowercase() == "home" {
            let source = r#"
                function view_Home() {
                    Text { content: "Universal Browser" }
                    Text { content: "Navigateur pour sites .unl" }
                    Button { content: "Visiter example.unl" }
                    Text { content: "Saisissez une URL dans la barre ci-dessus" }
                    Text { content: "Format: unl://nom_du_site" }
                }
            "#;
            let tokens = lexer::lex(source);
            let functions = parser::parse(&tokens);
            runtime.load(functions);
            runtime.set_view("Home");
            println!("✅ Page d'accueil du navigateur chargée");
            return;
        }

        // Convertir le protocole UNL en requête HTTP au serveur web_unl
        let server_url = format!("http://localhost:8080/project/{}", sanitized_domain);
        println!("🔄 Traduction interne: {} -> {}", sanitized_url, server_url);

        match reqwest::blocking::get(&server_url) {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(source) = response.text() {
                        println!("📄 Contenu .unl reçu ({} octets)", source.len());
                        let tokens = lexer::lex(&source);
                        let functions = parser::parse(&tokens);

                        if !functions.is_empty() {
                            runtime.load(functions);
                            runtime.set_view(&capitalize_first(domain));
                            println!("✅ Site unl:// chargé: {}", domain);
                        } else {
                            println!("❌ Erreur: aucune fonction trouvée dans le fichier .unl");
                        }
                    } else {
                        println!("❌ Erreur: impossible de lire le contenu .unl");
                    }
                } else {
                    println!("❌ Erreur: site .unl introuvable ({})", response.status());
                }
            }
            Err(e) => {
                println!("❌ Erreur de connexion au serveur UNL: {}", e);
            }
        }
    } else {
        println!("❌ URL non valide, doit commencer par 'unl://'");
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
