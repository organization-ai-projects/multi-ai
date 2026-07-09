# Low Level Shared Knowledge Module

Ce module contient les composants de bas niveau spécifiques à la gestion de la connaissance partagée.

## Composants prévus

- `knowledge_node_definition.rs` - Définition des nœuds de connaissance partagée
- `knowledge_relation_definition.rs` - Définition des relations entre les nœuds
- `knowledge_indexing_system.rs` - Système d'indexation pour optimiser les recherches
- `knowledge_in_memory_graph.rs` - Implémentation du graphe de connaissance en mémoire vive

Cette séparation entre définition (traits) et implémentation concrète permet:
- Une meilleure testabilité via les mocks/stubs
- Un découplage plus fort entre le contrat d'utilisation et les détails d'implémentation
- La possibilité d'avoir plusieurs implémentations pour une même définition

## Organisation des fichiers

Ce module adopte une organisation claire séparant les définitions de leurs implémentations :

```
low_level/
├── traits/
│   ├── knowledge_node.rs       # Traits définissant les nœuds
│   ├── knowledge_relation.rs   # Traits définissant les relations
│   ├── knowledge_indexing.rs   # Traits du système d'indexation
│   └── knowledge_graph.rs      # Traits du graphe de connaissance
├── impl/
│   ├── knowledge_node.rs       # Implémentation des nœuds
│   ├── knowledge_relation.rs   # Implémentation des relations
│   ├── knowledge_indexing.rs   # Implémentation du système d'indexation
│   └── knowledge_graph.rs      # Implémentation du graphe en mémoire
└── mod.rs                      # Expose uniquement les traits, pas les implémentations
```

Cette organisation offre plusieurs avantages :
- Séparation claire entre interface et implémentation
- Noms explicites décrivant précisément le rôle de chaque composant
- Structure de fichiers plus simple et moins fragmentée
- Facilité pour ajouter de nouvelles implémentations alternatives
- Meilleure visibilité sur l'architecture logique du module

## Règles de développement strictes

Les fichiers dans ce module `low_level/` :
- N'importent **aucun fichier externe au sous-domaine**
- N'appellent **aucune logique métier**
- Exposent uniquement des **types**, des **algorithmes purs**, ou des **opérations atomiques**

Ces règles garantissent que ces composants restent purement techniques, sans dépendances métier.

## Responsabilités

- Définition des structures fondamentales pour représenter la connaissance partagée
- Opérations CRUD de bas niveau sur les nœuds et arêtes de connaissance
- Optimisation des performances d'accès et de modification
- Sérialisation/désérialisation des structures de connaissance

## Gestion de la persistance

Ce module ne gère PAS la persistance directement :
- Les structures et opérations définies ici sont purement in-memory
- La persistance est entièrement déléguée au sous-domaine `persistence/`
- Aucune logique de stockage ou chargement n'est présente dans ce module
- Les composants sont conçus pour être facilement sérialisables, mais ne connaissent pas les mécanismes de sérialisation

Cette séparation respecte l'architecture globale où la persistance est centralisée et déclenchée uniquement par l'orchestrateur de niveau supérieur.
