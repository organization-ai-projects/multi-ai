use crate::classifier::Impact;

pub fn generate_changelog(version: &str, impact: &Impact, files: &[String]) -> String {
    let title = match impact {
        Impact::Major => "🚨 Breaking Changes",
        Impact::Minor => "✨ New Features",
        Impact::Patch => "🛠️ Bug Fixes",
    };

    let mut result = format!("# Version {}\n\n## {}\n", version, title);

    for file in files {
        result.push_str(&format!("- `{}` modifié\n", file));
    }

    result
}
