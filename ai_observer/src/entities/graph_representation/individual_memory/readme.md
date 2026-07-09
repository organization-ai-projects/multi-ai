# Individual Memory Module

Ce module gère la mémoire individuelle spécifique à chaque instance d'IA.

## Composants

- **low_level/** - Structures et opérations de base pour la gestion de la mémoire individuelle
- **managers/** - Gestionnaires de fonctionnalités pour la mémoire individuelle
- **orchestrator_individual_memory.rs** - Orchestrateur coordonnant les managers de ce sous-domaine

## Responsabilités

- Stocker et gérer les perceptions, expériences et connaissances spécifiques à une instance d'IA
- Fournir des mécanismes efficaces de stockage et récupération de souvenirs
- Gérer les processus de consolidation, oubli et renforcement de la mémoire
- Organiser la mémoire pour permettre l'inférence et l'apprentissage

## Isolation du module

Ce module est totalement isolé. Aucun autre sous-domaine ne peut accéder à ses composants sans passer par son orchestrateur.

## Interaction avec l'orchestrateur principal

L'orchestrateur de ce sous-domaine expose une API qui est utilisée par l'ai_orchestrator.rs pour accéder aux fonctionnalités de la mémoire individuelle.

## Caractéristiques distinctives

- **Personnalisation** - La mémoire est spécifique à chaque instance d'IA
- **Subjectivité** - Les informations stockées reflètent la perspective unique de l'instance
- **Temporalité** - Les souvenirs sont organisés avec une dimension temporelle
- **Contextualisation** - Les souvenirs sont liés à des contextes d'expérience spécifiques

## Flux de données

1. Les perceptions et expériences sont reçues depuis l'orchestrateur principal
2. Ces données sont traitées et intégrées dans la structure de mémoire
3. Les requêtes de récupération permettent d'accéder aux souvenirs pertinents
4. Les connaissances validées peuvent être proposées à la connaissance partagée

Cette mémoire individuelle est le fondement de l'identité et de l'apprentissage unique de chaque instance d'IA.

## Interface publique de l'orchestrateur

L'orchestrateur `orchestrator_individual_memory.rs` expose les méthodes publiques suivantes:

### Méthodes de gestion du cycle de vie
- `new() -> Self` - Crée une nouvelle instance de l'orchestrateur
- `initialize(&mut self) -> Result<(), Error>` - Initialise les composants internes
- `shutdown(&mut self) -> Result<(), Error>` - Ferme proprement les ressources

### Opérations de mémoire
- `add_memory_node(&mut self, content: MemoryContent) -> Result<NodeId, Error>` - Crée un nouveau nœud mémoire
- `create_relation(&mut self, from: NodeId, to: NodeId, relation_type: RelationType) -> Result<EdgeId, Error>` - Établit une relation entre nœuds
- `expose_memory(&mut self, node_id: NodeId, exposure_level: ExposureLevel) -> Result<(), Error>` - Expose un souvenir pour renforcement
- `find_memories(&mut self, query: MemoryQuery) -> Result<Vec<MemoryNode>, Error>` - Recherche des souvenirs selon critères

### Gestion du cycle mémoire
- `promote_to_medium_term(&mut self, node_ids: &[NodeId]) -> Result<(), Error>` - Promeut des souvenirs vers la mémoire à moyen terme
- `promote_to_long_term(&mut self, node_ids: &[NodeId]) -> Result<(), Error>` - Promeut des souvenirs vers la mémoire à long terme
- `run_memory_lifecycle(&mut self) -> Result<MemoryLifecycleStats, Error>` - Exécute un cycle de maintenance mémoire

### Analyse et statistiques
- `get_memory_statistics(&self) -> Result<MemoryStatistics, Error>` - Obtient des statistiques sur l'état de la mémoire
- `analyze_memory_network(&self) -> Result<MemoryNetworkAnalysis, Error>` - Analyse la structure du réseau mémoriel

Cette interface publique est la seule façon d'interagir avec la mémoire individuelle depuis l'orchestrateur principal.
