use super::{types::*, backup, update, utils, content};

pub struct DomainAnalysis {
    pub modules: Vec<DetectedModule>,
    pub tracker: ModificationTracker,
}

impl DomainAnalysis {
    pub fn has_changes(&self) -> bool {
        self.tracker.total_changes > 0
    }
}

pub fn analyze_domain(domain_path: &str) -> std::io::Result<DomainAnalysis> {
    let modules = utils::collect_domains(domain_path)?;
    let mod_rs_content = content::generate_mod_content(&modules);
    let trait_enum_content = content::generate_enum_content(&modules);
    let mut tracker = ModificationTracker::default();
    update::check_modifications(domain_path, &mod_rs_content, &trait_enum_content, &mut tracker)?;

    Ok(DomainAnalysis { modules, tracker })
}

pub fn apply_changes(domain_path: &str) -> std::io::Result<()> {
    let analysis = analyze_domain(domain_path)?;

    if !analysis.has_changes() {
        return Ok(());
    }

    // Générer les contenus à écrire
    let mod_rs_content = if analysis.tracker.mod_rs_changes {
        Some(content::generate_mod_content(&analysis.modules))
    } else {
        None
    };
    let trait_enum_content = if analysis.tracker.trait_commands_changes {
        Some(content::generate_enum_content(&analysis.modules))
    } else {
        None
    };

    backup::create_backups(domain_path)?;

    if update::apply_changes(
        domain_path,
        &analysis.tracker,
        mod_rs_content.as_deref(),
        trait_enum_content.as_deref()
    )? {
        backup::cleanup_backups(domain_path)?;
        println!("✅ {} modification(s) appliquée(s)", analysis.tracker.total_changes);
    } else {
        println!("⚠️ Certaines modifications ont échoué");
        println!("ℹ️ Les sauvegardes ont été conservées");
    }

    Ok(())
}
