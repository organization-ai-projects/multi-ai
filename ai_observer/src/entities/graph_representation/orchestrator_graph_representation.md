# Graph Representation Orchestrator

Ce fichier définit l'orchestrateur principal du domaine de représentation graphique qui coordonne tous les sous-domaines.

## Responsabilités

- Coordonner les interactions entre les sous-domaines (individual_memory, shared_knowledge, persistence)
- Exposer une API unifiée à l'orchestrateur principal du système (ai_orchestrator)
- Gérer les opérations qui impliquent plusieurs sous-domaines
- Assurer la cohérence des données entre les différents sous-domaines
- Déclencher les opérations de persistance via le sous-domaine dédié

## Structure interne

L'orchestrateur du domaine graph_representation est structuré comme suit:

```rust
pub struct OrchestratorGraphRepresentation {
    // Orchestrateurs des sous-domaines
    individual_memory_orchestrator: OrchestratorIndividualMemory,
    shared_knowledge_orchestrator: OrchestratorSharedKnowledge,
    persistence_orchestrator: OrchestratorPersistence,
    
    // Configuration
    config: GraphRepresentationConfig,
    
    // État interne
    is_initialized: bool,
    error_state: Option<String>,
}
```

### Principe de fonctionnement

1. Toutes les méthodes publiques de l'orchestrateur de domaine délèguent leurs opérations aux orchestrateurs des sous-domaines appropriés
2. Pour les opérations impliquant plusieurs sous-domaines, l'orchestrateur de domaine coordonne la séquence d'appels
3. L'orchestrateur gère les transactions qui impliquent plusieurs sous-domaines pour garantir la cohérence
4. La persistance est toujours gérée au niveau du domaine, jamais directement par les sous-domaines

### Interaction avec les sous-domaines

L'orchestrateur n'interagit **JAMAIS** avec:
- Les managers des sous-domaines
- Les composants low_level des sous-domaines  
- Tout autre composant qui n'est pas un orchestrateur de sous-domaine

Cette restriction garantit une séparation claire des responsabilités et une architecture propre.

## Interface publique

L'orchestrateur `orchestrator_graph_representation.rs` expose les méthodes publiques suivantes:

### Méthodes de gestion du cycle de vie
- `new(config: GraphRepresentationConfig) -> Self` - Crée une nouvelle instance de l'orchestrateur
- `initialize(&mut self) -> Result<(), Error>` - Initialise tous les sous-domaines
- `shutdown(&mut self) -> Result<(), Error>` - Ferme proprement toutes les ressources

### Opérations sur la mémoire individuelle
- `add_individual_memory(&mut self, ai_id: AiId, content: MemoryContent) -> Result<NodeId, Error>` - Ajoute un souvenir à la mémoire d'une IA
- `query_individual_memory(&self, ai_id: AiId, query: MemoryQuery) -> Result<Vec<MemoryNode>, Error>` - Interroge la mémoire d'une IA
- `memory_lifecycle_maintenance(&mut self, ai_id: AiId) -> Result<(), Error>` - Exécute la maintenance de la mémoire

### Opérations sur la connaissance partagée
- `propose_shared_knowledge(&mut self, ai_id: AiId, content: KnowledgeContent) -> Result<ProposalId, Error>` - Propose une connaissance pour partage
- `validate_knowledge(&mut self, ai_id: AiId, proposal_id: ProposalId) -> Result<ValidationStatus, Error>` - Valide une connaissance partagée
- `query_shared_knowledge(&self, query: KnowledgeQuery) -> Result<Vec<KnowledgeNode>, Error>` - Interroge la base de connaissances partagées

### Opérations de persistance
- `save_all_data(&mut self) -> Result<(), Error>` - Sauvegarde l'état complet du système
- `load_all_data(&mut self) -> Result<(), Error>` - Charge l'état complet du système
- `export_data(&mut self, format: ExportFormat) -> Result<ExportedData, Error>` - Exporte les données du système
- `import_data(&mut self, data: ExportedData) -> Result<ImportStats, Error>` - Importe des données externes

### Opérations de haut niveau
- `create_memory_from_perception(&mut self, ai_id: AiId, perception: Perception) -> Result<NodeId, Error>` - Traite une perception et crée un souvenir
- `extract_knowledge_from_memory(&mut self, ai_id: AiId, memory_ids: Vec<NodeId>) -> Result<ProposalId, Error>` - Extrait une connaissance depuis des souvenirs
- `integrate_shared_knowledge_to_memory(&mut self, ai_id: AiId, knowledge_id: NodeId) -> Result<NodeId, Error>` - Intègre une connaissance partagée dans la mémoire individuelle

Cette interface constitue le seul point d'entrée pour interagir avec le domaine de représentation graphique.
