use crate::ai::manager_ai::launch_all::launch_all;
use clap::Command;

pub fn get_launch_command(project_base: &str) -> Command {
    Command::new("launch")
        .about("Lance toutes les IA actives définies dans le fichier de configuration")
        .action(|| {
            if let Err(e) = launch_all(project_base) {
                eprintln!("❌ Erreur : {}", e);
            }
        })
}
