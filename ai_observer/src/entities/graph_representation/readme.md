# Graph Representation Module

Ce module contient les sous-domaines liés à la représentation graphique des connaissances de l'IA.

## Sous-domaines inclus

- **individual_memory** - Gestion de la mémoire individuelle de l'IA
- **shared_knowledge** - Gestion de la connaissance partagée entre les instances d'IA
- **persistence** - Infrastructure centralisée pour la persistance des données de graphe

## Responsabilités du module

- Fournir une représentation graphique des connaissances
- Gérer la persistance et l'accès aux données de connaissances
- Offrir des capacités de requête et d'inférence sur les graphes de connaissances
- Maintenir la cohérence entre la mémoire individuelle et la connaissance partagée

## Interface avec l'orchestrateur principal

Le module expose ses fonctionnalités à travers les orchestrateurs de chaque sous-domaine, qui sont ensuite coordonnés par l'ai_orchestrator.rs principal. Pour la persistance, seul l'orchestrateur graph_representation peut invoquer les opérations de sauvegarde et chargement.

## Exemple d'utilisation

```rust
// Exemple de code montrant comment l'ai_orchestrator utilise les orchestrateurs de sous-domaine
```

## Distinction entre mémoire individuelle et connaissance partagée

### Mémoire individuelle
La mémoire individuelle représente les connaissances, expériences et perceptions spécifiques à une instance d'IA particulière. Elle contient :

- **Perceptions personnelles** : Observations et interactions directes de l'IA avec son environnement
- **Souvenirs épisodiques** : Séquences d'événements vécus par cette instance spécifique
- **Contexte personnel** : État interne, préférences et historique d'interactions
- **Connaissances privées** : Informations qui n'ont pas été validées ou partagées avec d'autres instances

La mémoire individuelle est persistante, subjective et privée à chaque instance d'IA.

### Connaissance partagée
La connaissance partagée constitue la base commune d'informations accessible et utilisable par toutes les instances d'IA du système. Elle contient :

- **Faits validés** : Informations vérifiées et acceptées par consensus entre les instances
- **Modèles conceptuels** : Représentations abstraites du monde partagées entre les instances
- **Connaissances collectives** : Agrégation des découvertes de chaque instance après validation
- **Ontologies et taxonomies** : Structures de classification communes

La connaissance partagée est persistante, objective et constitue une source de vérité commune pour toutes les instances.

## Flux d'information entre les sous-domaines

1. L'IA perçoit de nouvelles informations qui sont d'abord stockées dans sa mémoire individuelle
2. Après analyse et validation, certaines informations peuvent être proposées pour intégration dans la connaissance partagée
3. Le sous-domaine shared_knowledge applique des mécanismes de consensus pour valider ces informations
4. Une fois validées, ces informations deviennent accessibles à toutes les instances d'IA
5. Les instances d'IA peuvent enrichir leur mémoire individuelle avec des éléments de la connaissance partagée

Cette séparation permet d'équilibrer l'apprentissage individuel avec l'intelligence collective.