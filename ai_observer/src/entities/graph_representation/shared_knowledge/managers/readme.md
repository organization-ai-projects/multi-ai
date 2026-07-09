# Managers de la Connaissance Partagée

Ce module contient les gestionnaires de niveau intermédiaire qui orchestrent les opérations sur la connaissance partagée entre les instances d'IA.

## Composants prévus

- `knowledge_node_manager.rs` - Gestion des nœuds de connaissance partagée (création, mise à jour, validation)
- `consensus_manager.rs` - Gestion des mécanismes de consensus entre instances d'IA
- `query_manager.rs` - Gestion des requêtes complexes sur la connaissance partagée
- `ontology_manager.rs` - Orchestration des opérations sur les ontologies et taxonomies communes

## Responsabilités

- Exposer des opérations de haut niveau pour manipuler la connaissance partagée
- Implémenter la logique métier spécifique à la connaissance collective
- Coordonner l'utilisation des composants bas niveau
- Assurer la cohérence et l'intégrité des données partagées

## Interface

- Chaque manager expose une API claire pour l'orchestrateur
- Les managers n'interagissent pas directement entre eux
- Toute communication entre managers passe par l'orchestrateur
- Les managers accèdent aux composants de bas niveau via leurs interfaces publiques

## Politique de persistance

- Les managers ne déclenchent jamais eux-mêmes de persistance
- Toute opération de sauvegarde passe exclusivement par `orchestrator_shared_knowledge.rs`
- Les managers fournissent uniquement les données à persister, sans connaître les mécanismes de persistance
- Cette règle garantit une séparation claire des responsabilités et facilite les tests unitaires

## Intégration avec l'orchestrateur

- L'orchestrateur `orchestrator_shared_knowledge.rs` instancie et coordonne tous les managers
- Les managers sont injectés dans l'orchestrateur via des traits spécifiques
- Les dépendances externes (services, configuration) sont fournies par l'orchestrateur

## Distinction entre managers et composants low_level

Il est important de noter que les managers ne contiennent pas l'implémentation bas niveau des structures de données mais orchestrent les opérations sur ces structures:

- Les structures de données des ontologies et taxonomies sont définies dans le dossier `low_level/`
- Le manager `ontology_manager.rs` utilise ces structures pour offrir des opérations de plus haut niveau
- Le manager combine plusieurs opérations atomiques du niveau bas pour implémenter des fonctionnalités complètes

Cette séparation assure que la logique métier (dans les managers) reste distincte des structures de données et opérations primitives (dans `low_level/`).

## Modèle de connaissance partagée

Le système de connaissance partagée implémente un modèle différent mais complémentaire à celui de la mémoire individuelle:

### Niveaux de validation
- **Connaissances proposées** : Informations soumises par des instances IA, en attente de validation
- **Connaissances en cours de validation** : Informations en processus de vérification par consensus
- **Connaissances validées** : Informations acceptées et intégrées dans le socle commun

### Système de consensus
- Chaque connaissance proposée requiert une validation par plusieurs instances
- Le `consensus_manager.rs` implémente différents mécanismes de validation selon le type d'information
- Les validations sont pondérées selon la réputation et l'expertise des instances validantes

### Structure ontologique
- Les connaissances sont organisées selon des ontologies évolutives
- Les relations sont typées selon leur nature sémantique
- L'`ontology_manager.rs` maintient la cohérence globale de la structure

### Relations de confiance
- Chaque connaissance possède un score de confiance global
- Les relations entre connaissances ont des scores de fiabilité
- Ces scores évoluent avec l'accumulation de preuves ou contre-exemples
- Les scores peuvent déclencher des réévaluations automatiques

### Conservation des connaissances
- **Aucun pruning** : Les connaissances partagées, même obsolètes ou contredites, ne sont jamais supprimées
- Les connaissances peuvent être marquées comme "dépassées" ou "contredites" mais restent accessibles
- L'historique complet des évolutions de la connaissance est préservé
- Cette approche permet une traçabilité totale de l'évolution des connaissances et une exploration éventuelle des chemins alternatifs

## Différences clés avec la mémoire individuelle

1. **Orientation consensus** : Contrairement à la mémoire individuelle qui est subjective, la connaissance partagée exige validation et consensus
2. **Stabilité temporelle** : Les connaissances partagées sont plus stables et moins volatiles
3. **Structure ontologique** : Organisation plus formelle suivant des principes ontologiques
4. **Traçabilité** : Chaque connaissance conserve la trace de ses sources et validations

## Considérations techniques

- Gestion soigneuse des conflits lors des mises à jour concurrentes
- Support pour la validation distribuée des connaissances
- Performances optimisées pour les requêtes fréquentes
- Mécanismes robustes pour assurer la cohérence entre toutes les instances
