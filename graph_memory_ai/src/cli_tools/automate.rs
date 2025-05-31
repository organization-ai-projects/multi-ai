use super::cli_commands::execute_command;

pub fn run_automate_projects(args: &[&str]) -> Result<String, String> {
    let output = execute_command("automate_projects", args)?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(format!("automate_projects a échoué:\n{}", stderr));
    }
    Ok(stdout.to_string())
}
