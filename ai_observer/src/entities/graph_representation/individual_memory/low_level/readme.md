# Low Level Individual Memory Module

Ce module contient les composants de bas niveau spécifiques à la gestion de la mémoire individuelle de l'IA.

## Composants prévus

Ce module est organisé en dossiers `traits/` et `impl/` avec les fichiers suivants dans chacun :

- `memory_node.rs` - Définition et implémentation des nœuds de mémoire individuelle
- `memory_relation.rs` - Définition et implémentation des relations entre les nœuds
- `memory_indexing.rs` - Système d'indexation pour accélérer les recherches dans le graphe
- `memory_structure.rs` - Structures de données pour l'organisation efficace en mémoire vive

Cette séparation entre définition (traits) et implémentation concrète permet:
- Une meilleure testabilité via les mocks/stubs
- Un découplage plus fort entre le contrat d'utilisation et les détails d'implémentation
- La possibilité d'avoir plusieurs implémentations pour une même définition

## Organisation des fichiers

Ce module adopte une organisation claire séparant les définitions de leurs implémentations :

```
low_level/
├── traits/
│   ├── memory_node.rs          # Traits définissant les nœuds
│   ├── memory_relation.rs      # Traits définissant les relations
│   ├── memory_indexing.rs      # Traits pour l'indexation et l'accélération des recherches
│   └── memory_structure.rs     # Traits pour l'organisation des données en mémoire vive
├── impl/
│   ├── memory_node.rs          # Implémentation des nœuds
│   ├── memory_relation.rs      # Implémentation des relations
│   ├── memory_indexing.rs      # Implémentation du système d'indexation
│   └── memory_structure.rs     # Implémentation des structures mémorielles fondamentales
└── mod.rs                      # Expose uniquement les traits, pas les implémentations
```

**Note importante**: Le graphe de mémoire complet n'est PAS implémenté au niveau low_level, mais plutôt construit au niveau des managers en composant ces éléments fondamentaux. Les composants low_level fournissent uniquement les "briques" élémentaires (nœuds, relations, indexation, conteneurs), respectant ainsi la séparation stricte des responsabilités.

Cette organisation offre plusieurs avantages :
- Séparation claire entre interface et implémentation
- Noms explicites décrivant précisément le rôle de chaque composant
- Structure de fichiers plus simple et moins fragmentée
- Facilité pour ajouter de nouvelles implémentations alternatives
- Meilleure visibilité sur l'architecture logique du module

## Responsabilités

- Définition des structures fondamentales pour représenter la mémoire individuelle
- Opérations CRUD de bas niveau sur les nœuds et arêtes de mémoire
- Optimisation des performances d'accès et de modification de la mémoire
- Sérialisation/désérialisation des structures mémorielles

## Règles de développement strictes

Les fichiers dans ce module `low_level/` :
- N'importent **aucun fichier externe au sous-domaine**
- N'appellent **aucune logique métier**
- Exposent uniquement des **types**, des **algorithmes purs**, ou des **opérations atomiques**

Ces règles garantissent une séparation claire des préoccupations et permettent de tester ces composants de manière isolée.

## Interface

- Les managers du module individual_memory interagissent avec ce niveau via des traits clairement définis
- Chaque composant expose une API minimale et bien documentée
- Aucune dépendance vers les managers n'est autorisée à ce niveau

## Considérations techniques

- Les structures doivent être thread-safe
- La performance est critique, particulièrement pour les opérations de requête
- Les modifications de la mémoire doivent maintenir l'intégrité des données
- La consommation mémoire doit être optimisée pour des graphes potentiellement larges

## Relation avec la persistance

Ce module ne gère PAS la persistance directement :
- Les composants low_level définissent uniquement les structures de données et leurs opérations in-memory
- Aucune implémentation de persistance n'est présente dans ce module
- Toute persistance est déléguée au sous-domaine `persistence/` via l'orchestrateur approprié
- Les structures définies ici exposent seulement des méthodes pour faciliter leur sérialisation/désérialisation

Cette séparation est conforme à la politique globale où seul `orchestrator_graph_representation.rs` peut déclencher des opérations de persistance via le sous-domaine dédié.