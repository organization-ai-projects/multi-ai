use super::types::*;

pub fn generate_enum_content(domains: &[DetectedModule]) -> String {
    let mut domains: Vec<_> = domains.iter()
        .filter(|m| matches!(m.module_type, ModuleType::Domain(_)))
        .map(|m| m.name.to_uppercase())
        .collect();
    domains.sort();

    let items = if domains.len() == 1 {
        format!("    {}", domains[0])
    } else {
        domains.iter()
            .map(|d| format!("    {},", d))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!("pub enum CommandDomain {{\n{}\n}}", items)
}

pub fn generate_mod_content(modules: &[DetectedModule]) -> String {
    let mut new_mods: Vec<String> = modules.iter()
        .map(|m| format!("pub mod {};", m.name))
        .collect();
    new_mods.sort();
    new_mods.join("\n")
}
