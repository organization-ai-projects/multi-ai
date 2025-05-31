use clap::Parser;
use std::path::PathBuf;
use std::io::{self, Write};
use crate::cli_domains_manager::{coordinator, ModuleType};

#[derive(Parser, Debug)]
#[command(author, version, about = "Générateur de registre de domaines CLI")]
pub struct Args {
    #[arg(short, long)]
    pub verbose: bool,

    /// Tester sans modifier les fichiers
    #[arg(long)]
    pub dry_run: bool,

    /// Appliquer les modifications après validation
    #[arg(long)]
    pub apply: bool,
}

pub fn run(args: Args, project_path: &PathBuf) -> Result<(), String> {
    if args.apply && !args.dry_run {
        return Err("Le flag --apply nécessite --dry-run".to_string());
    }

    let config = crate::config::Config::load(project_path.join("cli_creator.ron"))?;
    
    if args.verbose {
        println!("📋 Analyse des projets configurés:");
    }

    for path in &config.paths {
        let domain_path = format!("{}/src/cli_commands", path);
        
        // Convertir les erreurs IO en String
        let analysis = coordinator::analyze_domain(&domain_path)
            .map_err(|e| format!("Erreur analyse : {}", e))?;
        
        if args.verbose {
            display_analysis(&domain_path, &analysis);
        }

        if analysis.has_changes() {
            if args.dry_run {
                display_changes(&analysis);
                if prompt_user_confirmation() {
                    coordinator::apply_changes(&domain_path)
                        .map_err(|e| format!("Erreur application : {}", e))?;
                }
            } else {
                coordinator::apply_changes(&domain_path)
                    .map_err(|e| format!("Erreur application : {}", e))?;
            }
        } else if args.verbose {
            println!("✓ Tous les fichiers sont à jour pour {}", path);
        }
    }
    Ok(())
}

fn display_analysis(path: &str, analysis: &coordinator::DomainAnalysis) {
    println!("\n🔍 Scan de: {}", path);

    // Filtrer les modules par type
    let domains: Vec<_> = analysis.modules.iter()
        .filter(|m| matches!(m.module_type, ModuleType::Domain(_)))
        .cloned()
        .collect();
    let system_files: Vec<_> = analysis.modules.iter()
        .filter(|m| matches!(m.module_type, ModuleType::System))
        .cloned()
        .collect();

    println!("\n1️⃣ Domaines détectés :");
    for module in &domains {
        println!("  • {}", module.name);
    }

    println!("\n2️⃣ Fichiers système :");
    for module in &system_files {
        println!("  • {}", module.name);
    }

    println!("\n📦 Structure détectée :");
    println!("├── cli_commands/");
    println!("│   ├── trait_commands.rs");
    for module in domains.iter().filter(|m| matches!(m.module_type, ModuleType::Domain(_))) {
        println!("│   ├── {}/", module.name);
        println!("│   │   ├── mod.rs");
        println!("│   │   ├── commands.rs");
        if let ModuleType::Domain(struct_name) = &module.module_type {
            println!("│   │   │   └── struct {}", struct_name);
        }
        println!("│   │   ├── create.rs");
        println!("│   │   └── launch.rs");
    }
    println!("│   └── mod.rs  (à modifier)");

    println!("\n📝 État des fichiers :");
    
    // 1. Vérification mod.rs principal
    println!("\n1. cli_commands/mod.rs :");
    let current = std::fs::read_to_string(&format!("{}/mod.rs", path))
        .unwrap_or_default();
    
    let mut new_content = String::new();
    for module in domains.iter().filter(|m| matches!(m.module_type, ModuleType::Domain(_))) {
        new_content.push_str(&format!("pub mod {};\n", module.name));
    }
    for module in system_files {
        new_content.push_str(&format!("pub mod {};\n", module.name));
    }

    if current.trim() == new_content.trim() {
        println!("  ✓ Déjà conforme, contenu actuel :\n");
        println!("{}", current.trim());
    } else {
        crate::cli_domains_manager::show_diff(&current, &new_content, "mod.rs");
    }

    // 2. Vérification des fichiers de domaine
    for module in domains.iter().filter(|m| matches!(m.module_type, ModuleType::Domain(_))) {
        println!("\n2. cli_commands/{}/mod.rs :", module.name);
        if let Ok(content) = std::fs::read_to_string(&format!("{}/{}/mod.rs", path, module.name)) {
            println!("  ✓ Existe et ne sera pas modifié");
            println!("  Contenu actuel :");
            println!("  {}", content.trim());
        }
    }

    // 3. Vérification de trait_commands.rs
    println!("\n3. cli_commands/trait_commands.rs :");
    if let Ok(current) = std::fs::read_to_string(&format!("{}/trait_commands.rs", path)) {
        let new_enum = crate::cli_domains_manager::generate_enum_content(&domains);
        let enum_pattern = r"pub enum CommandDomain \{[\s\S]*?\}";
        let re = regex::Regex::new(enum_pattern).unwrap();
        
        if let Some(current_enum) = re.find(&current) {
            if current_enum.as_str() == new_enum {
                println!("  ✓ Déjà conforme, enum actuel :\n");
                println!("{}", current_enum.as_str());
            } else {
                crate::cli_domains_manager::show_diff(current_enum.as_str(), &new_enum, "trait_commands.rs (enum CommandDomain)");
            }
        }
    }

    println!("\n----------------------------------------");
}

fn display_changes(analysis: &coordinator::DomainAnalysis) {
    println!("\n📝 Modifications nécessaires:");
    if analysis.tracker.mod_rs_changes {
        println!("  • mod.rs sera mis à jour");
    }
    if analysis.tracker.trait_commands_changes {
        println!("  • trait_commands.rs sera mis à jour");
    }
}

fn prompt_user_confirmation() -> bool {
    print!("\nVoulez-vous appliquer ces modifications ? [y/N] ");
    io::stdout().flush().unwrap();
    
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().eq_ignore_ascii_case("y")
}
