use super::{create, launch};
use crate::cli_commands::trait_commands::DomainCommands;
use clap::Command;

pub struct AiDomain;

impl DomainCommands for AiDomain {
    fn get_domain_name() -> &'static str {
        "ai"
    }

    fn get_domain_commands(project_base: &str) -> Command {
        Command::new("ai")
            .about("Gestion des intelligences artificielles")
            .subcommand(create::get_create_command())
            .subcommand(launch::get_launch_command(project_base))
    }
}
