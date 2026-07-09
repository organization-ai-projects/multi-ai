use clap::{Command, ArgMatches};
use crate::cli_commands::ai::commands::AiDomain;
use crate::cli_commands::trait_commands::DomainCommands;

pub struct Cli {
    pub project_base: String,
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new(project_base: &str) -> Self {
        let matches = Command::new("Rust Universal Manager")
            .version("1.0")
            .author("Votre Nom <votre.email@example.com>")
            .about("Gestionnaire universel")
            .subcommand(AiDomain::get_domain_commands(project_base))
            .get_matches();

        println!("🚀 Domaine chargé : {}", AiDomain::get_domain_name());

        Self {
            project_base: project_base.to_string(),
            matches,
        }
    }
}
