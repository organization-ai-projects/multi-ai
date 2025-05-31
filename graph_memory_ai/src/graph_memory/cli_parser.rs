#[derive(Default)]
pub struct CliOutput {
    pub domains: Vec<String>,
    pub system_files: Vec<String>,
    pub status: Option<RunStatus>,
    pub actions: Vec<String>,
    pub errors: Vec<String>,
}

pub struct RunStatus {
    pub status: String,
    pub success: bool,
    pub error_code: String,
    pub error_message: String,
}

pub fn parse_cli_output(output: &str) -> CliOutput {
    let mut parsed = CliOutput::default();
    
    parsed.domains = parse_section(output, "1️⃣ Domaines détectés :");
    parsed.system_files = parse_section(output, "2️⃣ Fichiers système :");
    parsed.status = parse_status(output);
    parsed.actions = parse_actions(output);
    parsed.errors = parse_errors(output);
    
    parsed
}

fn parse_section(output: &str, section_marker: &str) -> Vec<String> {
    output.split(section_marker)
        .nth(1)
        .map(|section| section.lines()
            .skip(1)
            .take_while(|l| !l.trim().is_empty())
            .filter(|l| l.starts_with("• "))
            .map(|l| l.trim_start_matches("• ").trim().to_string())
            .collect())
        .unwrap_or_default()
}

fn parse_status(output: &str) -> Option<RunStatus> {
    Some(if output.contains("✓ Tous les fichiers sont à jour") {
        RunStatus {
            status: "ok".into(),
            success: true,
            error_code: "".into(),
            error_message: "".into(),
        }
    } else if let Some(err) = output.lines().find(|l| l.contains("Erreur") || l.contains("❌")) {
        RunStatus {
            status: "error".into(),
            success: false,
            error_code: err.trim().into(),
            error_message: err.trim().into(),
        }
    } else if output.contains("⚠️") {
        RunStatus {
            status: "warning".into(),
            success: false,
            error_code: "warning".into(),
            error_message: "".into(),
        }
    } else {
        RunStatus {
            status: "unknown".into(),
            success: false,
            error_code: "".into(),
            error_message: "".into(),
        }
    })
}

fn parse_actions(output: &str) -> Vec<String> {
    output.lines()
        .filter(|l| l.contains("sera mis à jour"))
        .map(|l| l.trim().to_string())
        .collect()
}

fn parse_errors(output: &str) -> Vec<String> {
    output.lines()
        .filter(|l| l.contains("Erreur") || l.contains("❌"))
        .map(|l| l.trim().to_string())
        .collect()
}
