use std::io::{self, Write};

/// Lit une entrée textuelle avec un message
pub fn read_input(message: &str) -> String {
    print!("{}: ", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    input.trim().to_string()
}

/// Lit une entrée optionnelle (peut être vide)
pub fn read_input_optional(message: &str) -> Option<String> {
    let input = read_input(message);
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

/// Lit un choix de menu et retourne un entier
pub fn read_menu_choice() -> u32 {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Erreur de lecture");

    choice.trim().parse::<u32>().unwrap_or(0)
}

/// Demande une confirmation (o/n)
pub fn confirm(message: &str) -> bool {
    print!("{} (o/n): ", message);
    io::stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Erreur de lecture");

    let response = response.trim().to_lowercase();
    response == "o" || response == "oui" || response == "y" || response == "yes"
}

/// Affiche un message de succès
pub fn show_success(message: &str) {
    println!("✅ {}", message);
}

/// Affiche un message d'erreur
pub fn show_error(message: &str) {
    eprintln!("❌ {}", message);
}

/// Affiche un en-tête de section
pub fn show_section(title: &str) {
    println!("\n===== {} =====", title);
}

/// Affiche un message d'information
pub fn show_info(message: &str) {
    println!("ℹ️ {}", message);
}

/// Lire un nombre avec gestion d'erreurs
pub fn read_number<T: std::str::FromStr>(message: &str) -> Option<T> {
    let input = read_input(message);
    match input.parse::<T>() {
        Ok(num) => Some(num),
        Err(_) => {
            show_error("Valeur numérique invalide");
            None
        }
    }
}

/// Affiche une liste de résultats numérotée
pub fn show_numbered_list<T: std::fmt::Display>(items: &[T], empty_message: &str) {
    if items.is_empty() {
        show_info(empty_message);
    } else {
        for (i, item) in items.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }
    }
}
