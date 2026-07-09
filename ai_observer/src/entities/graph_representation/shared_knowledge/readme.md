# Shared Knowledge Module

Ce module gère la connaissance partagée entre différentes instances d'IA.

## Composants

- **low_level/** - Structures et opérations de base pour la gestion de la connaissance partagée
- **managers/** - Gestionnaires de fonctionnalités pour la connaissance partagée
- **orchestrator_shared_knowledge.rs** - Orchestrateur coordonnant les managers de ce sous-domaine

## Responsabilités

- Maintenir une base de connaissances commune accessible par toutes les instances d'IA
- Gérer les mécanismes de consensus pour la validation des connaissances partagées
- Assurer la cohérence et l'intégrité de la connaissance partagée
- Fournir des mécanismes d'accès et de mise à jour de la connaissance partagée

## Isolation du module

Ce module est totalement isolé. Aucun autre sous-domaine ne peut accéder à ses composants sans passer par son orchestrateur.

## Interaction avec l'orchestrateur principal

L'orchestrateur de ce sous-domaine expose une API qui est utilisée par l'ai_orchestrator.rs pour accéder aux fonctionnalités de la connaissance partagée.

## Interface publique de l'orchestrateur

L'orchestrateur `orchestrator_shared_knowledge.rs` expose les méthodes publiques suivantes:

### Méthodes de gestion du cycle de vie
- `new() -> Self` - Crée une nouvelle instance de l'orchestrateur
- `initialize(&mut self) -> Result<(), Error>` - Initialise les composants internes
- `shutdown(&mut self) -> Result<(), Error>` - Ferme proprement les ressources

### Opérations de base
- `propose_knowledge(&mut self, content: KnowledgeContent, source: SourceId) -> Result<ProposalId, Error>` - Propose une nouvelle connaissance
- `validate_proposal(&mut self, proposal_id: ProposalId, validator: ValidatorId) -> Result<ValidationStatus, Error>` - Valide une proposition
- `reject_proposal(&mut self, proposal_id: ProposalId, reason: String) -> Result<(), Error>` - Rejette une proposition
- `search_knowledge(&self, query: KnowledgeQuery) -> Result<Vec<KnowledgeNode>, Error>` - Recherche dans la base de connaissances

### Gestion ontologique
- `define_ontology_relation(&mut self, relation_type: OntologyRelationType) -> Result<(), Error>` - Définit un nouveau type de relation ontologique
- `classify_knowledge(&mut self, node_id: NodeId, classification: Classification) -> Result<(), Error>` - Classifie une connaissance
- `get_ontology_hierarchy(&self) -> Result<OntologyHierarchy, Error>` - Récupère la hiérarchie ontologique complète

### Statistiques et analyse
- `get_consensus_statistics(&self) -> Result<ConsensusStatistics, Error>` - Statistiques sur le processus de consensus
- `get_most_trusted_knowledge(&self, limit: usize) -> Result<Vec<KnowledgeNode>, Error>` - Récupère les connaissances les plus fiables
- `get_knowledge_validation_history(&self, node_id: NodeId) -> Result<ValidationHistory, Error>` - Historique de validation d'une connaissance

Cette interface publique est la seule façon d'interagir avec la connaissance partagée depuis l'orchestrateur principal.
