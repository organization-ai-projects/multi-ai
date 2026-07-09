use clap::Command;

#[derive(Debug)]
pub enum CommandDomain {
    AI
}

pub trait DomainCommands {
    fn get_domain_name() -> &'static str;
    fn get_domain_commands(project_base: &str) -> Command;
}
