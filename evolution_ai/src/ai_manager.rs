use crate::archives::ArchiveManager;
use crate::evolution::Evolution;
use crate::launcher::Launcher;
use crate::monitoring::Monitor;
use crate::persistence::Checkpoint;
use crate::project_manager::ProjectManager;
use crate::species::{Lineage, SpeciesType};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct AIManager {
    pub population_dir: String,
    lineages: HashMap<String, Lineage>,
    launcher: Launcher,
    project_manager: ProjectManager,
    evolution: Evolution,
    monitor: Monitor,
}

impl AIManager {
    pub fn new() -> Self {
        fs::create_dir_all("ai_population").unwrap();
        Self {
            population_dir: "ai_population".to_string(),
            lineages: HashMap::new(),
            launcher: Launcher::new(10),
            project_manager: ProjectManager::new("ai_population".to_string()),
            evolution: Evolution::new(),
            monitor: Monitor::new(),
        }
    }

    pub fn create_project(&self, id: usize, species: Option<SpeciesType>) -> std::io::Result<()> {
        self.project_manager.create_project(id, species)
    }

    pub fn run_all_async(&self) -> Vec<tokio::process::Child> {
        self.launcher.run_all_async(self.get_all_ais())
    }

    pub fn get_next_id(&self) -> usize {
        fs::read_dir(&self.population_dir)
            .unwrap()
            .filter_map(Result::ok)
            .count()
            + 1
    }

    pub fn replace_dead(&mut self, performers: &[(String, f64)]) {
        // 1. Détecter et archiver les morts
        let dead_ais = self.detect_dead_ais();
        for dead_ai in &dead_ais {
            self.archive_dead_ai(dead_ai);
            self.handle_replacement(dead_ai, performers);
        }

        // 2. Gérer les extinctions d'espèces
        self.handle_species_extinction(performers);
    }

    fn detect_dead_ais(&self) -> Vec<std::path::PathBuf> {
        fs::read_dir(&self.population_dir)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.join("death.ron").exists())
            .collect()
    }

    fn archive_dead_ai(&self, dead_path: &Path) {
        let archive_manager = ArchiveManager::new(self.population_dir.clone());
        if let Err(e) = archive_manager.archive_dead_ai(dead_path) {
            eprintln!("Erreur archivage IA morte {}: {}", dead_path.display(), e);
        }
    }

    fn handle_replacement(&mut self, dead_path: &Path, performers: &[(String, f64)]) {
        // Logique de remplacement existante...
        let dead_ai_name = dead_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        println!("Remplacement de l'IA morte: {}", dead_ai_name);

        // Tenter d'abord le crossover
        if let Some(new_ai) = self.try_crossover_replacement(performers) {
            println!("Remplacement par crossover: {}", new_ai);
            return;
        }

        // Sinon, tenter une mutation simple
        if let Some(new_ai) = self.try_mutation_replacement(performers) {
            println!("Remplacement par mutation: {}", new_ai);
            return;
        }

        // En dernier recours, créer une nouvelle IA aléatoire
        let new_id = self.get_next_id();
        if let Err(e) = self.create_project(new_id, None) {
            eprintln!("Erreur création nouvelle IA: {}", e);
        }
    }

    fn try_crossover_replacement(&self, performers: &[(String, f64)]) -> Option<String> {
        // Extraire la logique de crossover...
        let parents = self.evolution.select_parents(performers);

        if let Some((parent1, parent2)) = parents {
            if let (Some(lineage1), Some(lineage2)) =
                (self.get_lineage(&parent1), self.get_lineage(&parent2))
            {
                // Essayer le crossover inter-espèces
                if let Some(new_code) = self.evolution.try_interspecies_crossover(
                    (&parent1, &lineage1.species),
                    (&parent2, &lineage2.species),
                ) {
                    let new_id = self.get_next_id();
                    if let Err(e) = self.create_project(new_id, None) {
                        eprintln!("Erreur création projet: {}", e);
                        return None;
                    }
                    if let Err(e) = fs::write(
                        format!("{}/ai_{:05}/src/main.rs", self.population_dir, new_id),
                        new_code,
                    ) {
                        eprintln!("Erreur écriture code: {}", e);
                    }
                    return Some(format!("ai_{:05}", new_id));
                }
            }
        }

        None
    }

    fn try_mutation_replacement(&self, performers: &[(String, f64)]) -> Option<String> {
        if let Some((best_path, _)) = performers
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        {
            let source_path = format!("{}/src/main.rs", best_path);
            let new_id = self.get_next_id();
            if let Err(e) = self.project_manager.create_evolved_project(
                &source_path,
                new_id,
                &mut self.evolution,
            ) {
                eprintln!("Erreur création IA évoluée: {}", e);
                let _ = self.create_project(new_id, None);
            }
            return Some(format!("ai_{:05}", new_id));
        }

        None
    }

    fn handle_species_extinction(&self, performers: &[(String, f64)]) {
        // Logique d'extinction extraite de replace_dead
        let extinct_species: HashSet<_> = self
            .lineages
            .values()
            .filter(|l| !performers.iter().any(|(p, _)| p.contains(&l.ancestor_id)))
            .map(|l| l.species.clone())
            .collect();

        for species in extinct_species {
            // Utilisation directe du monitor sans stats
            self.monitor.log_extinction(&species);

            if let Some(ancestor) = self.find_best_ancestor(&species) {
                // Gérer le Result de revive_species
                if let Err(e) = self.revive_species(ancestor, &species, &mut self.evolution) {
                    eprintln!(
                        "Erreur lors de la renaissance de l'espèce {:?}: {}",
                        species, e
                    );
                }
            }
        }

        // Appelle replace_dead de Evolution pour gérer le remplacement
        if let Err(e) = self.evolution.replace_dead(self, performers) {
            eprintln!("Erreur lors du remplacement des IAs: {}", e);
        }
    }

    pub fn get_all_ais(&self) -> Vec<std::path::PathBuf> {
        fs::read_dir(&self.population_dir)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect()
    }

    pub fn get_top_performers(&self, count: usize) -> Vec<std::path::PathBuf> {
        let mut ais: Vec<_> = fs::read_dir(&self.population_dir)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect();

        ais.sort_by(|a, b| {
            let score_a = self.get_ai_score(a).unwrap_or(0.0);
            let score_b = self.get_ai_score(b).unwrap_or(0.0);
            score_b.partial_cmp(&score_a).unwrap()
        });

        ais.into_iter().take(count).collect()
    }

    fn get_ai_score(&self, path: &std::path::Path) -> Option<f64> {
        let content = fs::read_to_string(path.join("debug.ron")).ok()?;
        let memory = ron::from_str::<serde_json::Value>(&content).ok()?;
        memory.get("last_reward")?.as_f64()
    }

    pub fn get_lineage(&self, path: &str) -> Option<Lineage> {
        let lineage_path = format!("{}/lineage.ron", path);
        fs::read_to_string(lineage_path)
            .ok()
            .and_then(|content| ron::from_str(&content).ok())
    }

    pub fn get_lineages(&self) -> HashMap<String, Lineage> {
        self.lineages.clone()
    }

    pub fn get_performers(&self) -> Vec<(String, f64)> {
        self.get_all_ais()
            .iter()
            .filter_map(|path| {
                self.get_ai_score(path)
                    .map(|score| (path.to_string_lossy().to_string(), score))
            })
            .collect()
    }

    pub fn restore_from_checkpoint(&mut self, checkpoint: &Checkpoint) -> std::io::Result<()> {
        self.lineages = checkpoint.lineages.clone();

        // Restaurer les IAs depuis leur dernier état sauvegardé
        for (ai_id, lineage) in &checkpoint.lineages {
            let ai_path = format!("{}/{}", self.population_dir, ai_id);
            if !Path::new(&ai_path).exists() {
                self.create_project(
                    ai_id.replace("ai_", "").parse().unwrap(),
                    Some(lineage.species.clone()),
                )?;
            }
        }
        Ok(())
    }

    pub fn find_best_ancestor(&self, species: &SpeciesType) -> Option<String> {
        self.lineages
            .iter()
            .filter(|(_, l)| &l.species == species)
            .max_by_key(|(_, l)| l.mutations_survived)
            .map(|(id, _)| id.clone())
    }

    pub fn revive_species(
        &self,
        ancestor_id: String,
        species: &SpeciesType,
        evolution: &mut Evolution, // Et ici
    ) -> std::io::Result<()> {
        let new_id = self.get_next_id();
        self.create_project(new_id, Some(species.clone()))?;

        // Copier et muter le code de l'ancêtre
        let ancestor_path = format!("{}/{}/src/main.rs", self.population_dir, ancestor_id);
        let source = fs::read_to_string(ancestor_path)?;
        let mutated = evolution.mutate_code(&source);

        let new_path = format!("{}/ai_{:05}/src/main.rs", self.population_dir, new_id);
        fs::write(new_path, mutated)?;

        Ok(())
    }

    pub fn reset_population(&self) -> std::io::Result<()> {
        // Sauvegarder les IAs existantes
        let backup_dir = format!("backup_{}", chrono::Local::now().format("%Y%m%d_%H%M%S"));
        fs::rename(&self.population_dir, &backup_dir)?;

        // Créer un nouveau dossier population
        fs::create_dir_all(&self.population_dir)?;

        // Recréer les IAs avec des species aléatoires
        for id in 1..=100 {
            // Nombre fixe d'IAs pour le moment
            self.create_project(id, Some(SpeciesType::random()))?;
        }

        Ok(())
    }

    pub fn check_population(&self, ai_dir: &str) -> Vec<(String, f64)> {
        // Déplacer la logique de check_population depuis evolution.rs vers ici
        // car c'est une responsabilité de gestion des IAs
        todo!()
    }

    pub fn balance_species(&mut self, stats: &mut HashMap<SpeciesType, SpeciesStats>) {
        // Déplacer la logique de balance_species ici aussi
        todo!()
    }
}
