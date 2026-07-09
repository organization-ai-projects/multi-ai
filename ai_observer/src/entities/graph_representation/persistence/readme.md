# Common Persistence Infrastructure

Ce module fournit l'infrastructure commune pour la persistance des données dans tous les sous-domaines de graph_representation.

## Objectif

Centraliser les mécanismes génériques de persistance pour:
- Éviter la duplication de code entre sous-domaines
- Garantir une approche cohérente de la persistance
- Simplifier la maintenance et l'évolution des mécanismes de stockage

## Composants

- `persistence_traits.rs` - Traits communs pour toutes les opérations de persistance
- `serialization.rs` - Utilitaires de sérialisation/désérialisation communs
- `storage_engine.rs` - Moteur de stockage abstrait avec implémentations concrètes
- `format_converters.rs` - Convertisseurs entre formats .ron et .bin

## Formats standardisés

Tous les sous-domaines utilisent deux formats standardisés:
- **RON** (.ron) - Format texte lisible pour le débogage, l'inspection manuelle et la portabilité
- **Binaire** (.bin) - Format optimisé pour les performances lors des manipulations par les IA

## Utilisation par les sous-domaines

Chaque sous-domaine:
1. Importe et utilise les traits et utilités communes
2. Implémente ses propres spécificités de persistance
3. Peut étendre les traits communs selon ses besoins

## Configuration

Le module fournit également:
- Des options de configuration pour le stockage (chemins, taille des caches, etc.)
- Des mécaniques de migration pour la compatibilité ascendante
- Des utilitaires de diagnostic pour le débogage des problèmes de persistance

## Responsabilités

Ce module est responsable UNIQUEMENT des mécanismes génériques de persistance. La logique métier spécifique à chaque type de données reste dans les sous-domaines correspondants.

## Organisation

Ce sous-domaine est structuré en trois couches :

1. **low_level/** : composants fondamentaux indépendants du métier
2. **managers/** : logique métier de persistance propre au domaine graphe
3. **orchestrator_persistence.rs** : point d'entrée unique utilisé par le `orchestrator_graph_representation.rs`

## Règle métier

Les opérations de persistance ne peuvent être déclenchées que par l'orchestrateur du domaine `graph_representation`, jamais par les managers ou les composants internes.

## Isolation du module

Ce module est totalement isolé. Aucun autre sous-domaine ne peut accéder à ses composants sans passer par son orchestrateur. Cette isolation garantit une séparation claire des préoccupations et une meilleure testabilité.

## Architecture de persistance recommandée

L'architecture de persistance suit un modèle hybride à trois niveaux :

1. **Niveau infrastructure (ce module)** :
   - Définit les traits, interfaces et utilitaires communs
   - Fournit des implémentations de base réutilisables
   - Ne contient aucune logique métier spécifique à un domaine

2. **Niveau sous-domaine** :
   - Chaque sous-domaine implémente sa propre persistance via ses modules low_level
   - Utilise les traits et utilitaires du niveau infrastructure
   - Encapsule les détails spécifiques au domaine
   - Expose des opérations de persistance uniquement via son orchestrateur local

3. **Niveau orchestration globale** :
   - `ai_orchestrator.rs` coordonne les transactions multi-domaines
   - Gère les opérations de persistance qui touchent plusieurs sous-domaines
   - Assure la cohérence globale lors des opérations distribuées

Ce modèle hybride offre à la fois :
- L'**encapsulation** : chaque sous-domaine gère ses propres données
- La **cohérence** : infrastructure commune et standards partagés
- La **maintenabilité** : séparation claire des responsabilités
- L'**évolutivité** : possibilité d'ajouter/modifier des sous-domaines sans refactoring massif

## Politique de persistance

- La persistance est centralisée dans le sous-domaine `persistence/`.
- Seul `orchestrator_graph_representation.rs` a le droit d'appeler les opérations de sauvegarde, chargement ou export.
- Aucun `manager/` ni composant `low_level/` ne doit connaître ou invoquer directement une logique de persistance.
- Cette règle garantit une orchestration claire, un contrôle total du cycle de vie des données, et une testabilité maximale.

## Pourquoi la persistance est locale au domaine

La persistance est intégrée dans `graph_representation/` car elle dépend directement de la structure des données manipulées (graphe).  
Les règles d'encodage, les formats supportés, et les opérations de sauvegarde sont spécifiques à la représentation sous forme de graphe de connaissance.  
Centraliser cette logique permet :

- Une cohérence forte entre structure et stockage
- Une encapsulation métier totale
- Une évolutivité vers d'autres représentations (symbolique, vectorielle...) avec leur propre persistance locale

## Flux typique d'une opération de persistance

1. L'application appelle une méthode sur `ai_orchestrator.rs`
2. Si l'opération concerne un seul sous-domaine, elle est déléguée à l'orchestrateur local
3. L'orchestrateur local utilise ses managers pour effectuer l'opération
4. Les managers utilisent les composants low_level qui implémentent la persistance

Pour les opérations multi-domaines :
1. `ai_orchestrator.rs` coordonne la séquence d'appels aux orchestrateurs locaux
2. Il gère les transactions distribuées si nécessaire
3. Il assure la cohérence globale des données

## Avantages à long terme

Cette architecture :
- Supporte la **mise à l'échelle** vers des systèmes distribués
- Facilite l'**ajout de nouveaux sous-domaines** sans modifier l'existant
- Permet l'**évolution indépendante** des sous-domaines
- Maintient une **base de code cohérente** et bien structurée

## Interface publique de l'orchestrateur

L'orchestrateur `orchestrator_persistence.rs` expose les méthodes publiques suivantes:

### Méthodes de gestion du cycle de vie
- `new(config: PersistenceConfig) -> Self` - Crée une nouvelle instance de l'orchestrateur
- `initialize(&mut self) -> Result<(), Error>` - Initialise les composants internes
- `shutdown(&mut self) -> Result<(), Error>` - Ferme proprement les ressources

### Opérations de persistance
- `save_graph_data(&mut self, graph_type: GraphType, data: &GraphData) -> Result<(), Error>` - Sauvegarde les données d'un graphe spécifique
- `load_graph_data(&mut self, graph_type: GraphType) -> Result<GraphData, Error>` - Charge les données d'un graphe spécifique
- `backup_graph_data(&mut self, graph_type: GraphType) -> Result<BackupId, Error>` - Crée une sauvegarde d'un graphe
- `restore_backup(&mut self, backup_id: BackupId) -> Result<(), Error>` - Restaure depuis une sauvegarde

### Export/Import
- `export_graph_data(&mut self, graph_type: GraphType, format: ExportFormat) -> Result<ExportedData, Error>` - Exporte des données de graphe
- `import_graph_data(&mut self, graph_type: GraphType, data: ExportedData) -> Result<ImportStats, Error>` - Importe des données de graphe
- `migrate_data(&mut self, from_version: Version, to_version: Version) -> Result<MigrationStats, Error>` - Migre des données

### Maintenance et statistiques
- `run_consistency_check(&mut self) -> Result<ConsistencyReport, Error>` - Vérifie la cohérence des données
- `get_storage_statistics(&self) -> Result<StorageStatistics, Error>` - Obtient des statistiques sur le stockage
- `cleanup_temporary_files(&mut self) -> Result<CleanupStats, Error>` - Nettoie les fichiers temporaires

Cette interface publique est la seule façon d'interagir avec le système de persistance depuis l'orchestrateur de représentation graphique.
