//rust_universal/src/ai/manager_ai/create_agent.rs
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn create_agent() -> anyhow::Result<()> {
    let template_path = PathBuf::from("ai/ai_base");
    let agent_id = Uuid::new_v7().to_string();
    let ai_name = format!("agent_{}", agent_id);
    let agent_path = PathBuf::from("ai/agents").join(&ai_name);

    if agent_path.exists() {
        eprintln!("❌ Agent {} already exists", ai_name);
        return Ok(());
    }

    create_dir_all(&agent_path)?;

    // Copier tout le contenu de ai_base dans le dossier de l'agent
    copy_dir_all(template_path.to_str().unwrap(), agent_path.to_str().unwrap())?;

    // metadata.ron (avec chemins relatifs)
    let metadata_content = format!(
        r#"
(
    id: "{agent_id}",
    name_short: "{ai_name}",
    name_full: "Intelligence Artificielle {ai_name}",
    description: "Description de l'IA {ai_name}",
    version: "0.1.0",
    paths: (
        base: ".",
        main: "./main.rs",
        config: "./config",
    ),
)
"#,
        agent_id = agent_id,
        ai_name = ai_name,
    );
    let metadata_path = agent_path.join("metadata.ron");
    let mut metadata_file = File::create(&metadata_path)?;
    metadata_file.write_all(metadata_content.as_bytes())?;

    println!("✅ Agent {} created successfully with ID {}", ai_name, agent_id);
    Ok(())
}

fn copy_dir_all(src: &str, dest: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = Path::new(dest).join(entry.file_name());

        if path.is_dir() {
            create_dir_all(&dest_path)?;
            copy_dir_all(path.to_str().unwrap(), dest_path.to_str().unwrap())?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}