/// Niveaux de sécurité pour le moniteur
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    /// Surveillance de base avec alertes
    Standard,
    /// Interventions automatiques en cas de problème
    High,
    /// Mode paranoïaque: termine immédiatement tout processus suspect
    Critical,
}

/// Type d'alerte de sécurité
#[derive(Debug, Clone)]
pub enum SecurityAlert {
    /// Nouvelle IA détectée
    NewAiDetected {
        path: String,
        created_by: Option<String>,
    },
    /// Modification de fichier non autorisée
    UnauthorizedFileModification {
        path: String,
        modified_by: Option<u32>,
    },
    /// Consommation excessive de ressources
    ExcessiveResourceUsage {
        ai_id: String,
        pid: u32,
        cpu_percent: f32,
        memory_mb: u64,
    },
    /// Création excessive de processus
    ExcessiveProcessCreation {
        ai_id: String,
        parent_pid: u32,
        child_count: usize,
    },
    /// Tentative d'accès à un chemin interdit
    ForbiddenPathAccess {
        path: String,
        ai_id: Option<String>,
        pid: Option<u32>,
    },
}

impl Default for SecurityLevel {
    fn default() -> Self {
        SecurityLevel::High
    }
}
