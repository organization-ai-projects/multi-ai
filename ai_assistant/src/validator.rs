use crate::classifier::Impact;
use semver_planner::version::SemVer; // Mise à jour de l'import

#[derive(Debug)]
pub struct ValidationReport {
    pub valid: bool,
    pub message: String,
}

pub fn validate_bump(before: &SemVer, after: &SemVer, expected: &Impact) -> ValidationReport {
    let actual = if after.major > before.major {
        Impact::Major
    } else if after.minor > before.minor {
        Impact::Minor
    } else if after.patch > before.patch {
        Impact::Patch
    } else {
        return ValidationReport {
            valid: false,
            message: "Aucune modification de version détectée.".to_string(),
        };
    };

    let valid = &actual == expected;
    let msg = if valid {
        format!("✅ Bump cohérent avec l'impact {:?}.", expected)
    } else {
        format!("⚠️ Bump incohérent. Attendu {:?}, obtenu {:?}.", expected, actual)
    };

    ValidationReport {
        valid,
        message: msg,
    }
}
