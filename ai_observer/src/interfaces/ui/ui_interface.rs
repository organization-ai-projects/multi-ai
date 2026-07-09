use crate::api::system_api::SystemAPI;
use std::collections::HashMap;

/// Interface graphique pour interagir avec le système IA
pub struct UIInterface {
    /// Référence à l'API système, identique à celle utilisée par la CLI
    api: SystemAPI,
}

impl UIInterface {
    /// Crée une nouvelle interface graphique
    pub fn new(api: SystemAPI) -> Self {
        UIInterface { api }
    }

    /// Démarre l'application graphique
    pub fn run(&mut self) {
        // Initialisation de la fenêtre principale de l'application
        println!("Interface graphique démarrée");

        // Ici, on initialiserait la boucle d'événements de l'UI
        // par exemple avec gtk-rs, egui, iced, etc.

        // Exemple simplifié:
        self.setup_ui();

        // Si l'UI utilise un thread séparé, on attendrait sa fin ici
    }

    /// Configure l'interface utilisateur
    fn setup_ui(&mut self) {
        // Ajouter les widgets et les connecter aux fonctions de l'API

        // Par exemple, pour l'ajout de concept:
        // - Le bouton "Ajouter concept" appellerait l'API via:
        //   self.api.create_concept(id, content, metadata)

        // - La visualisation du graphe utiliserait:
        //   let concepts = self.api.get_all_concepts();
        //   let relations = self.api.get_all_relations();
    }

    // Fonctions de délégation à l'API exactement comme dans CLIInterface
    // Ces fonctions seraient appelées par les callbacks des widgets

    /// Ajoute un concept via l'API
    pub fn create_concept(
        &mut self,
        id: &str,
        content: &str,
        metadata: HashMap<String, String>,
    ) -> Result<(), String> {
        self.api.create_concept(id, content, metadata)
    }

    // ... autres méthodes de délégation identiques à celles dans CLIInterface ...
}
