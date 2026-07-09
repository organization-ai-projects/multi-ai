# AI Observer - Entities Module

Ce module contient les entités et l'orchestration centrale du système d'IA Observer.

## Hiérarchie des responsabilités

1. **ai_orchestrator.rs** - Seul composant "transdomaine" qui coordonne tous les domaines
2. **orchestrator_[nom_du_sous_domaine].rs** - Coordonne uniquement les managers de son propre sous-domaine
3. **managers** - Connaissent uniquement les composants low_level de leur propre sous-domaine
4. **low_level** - Composants fondamentaux sans connaissance du reste du système

## Structure générale des sous-domaines

Chaque sous-domaine doit respecter la structure suivante:

```
[nom_du_sous_domaine]/
├── low_level/
│   └── [composants spécifiques au sous-domaine]
├── managers/
│   └── [managers spécifiques au sous-domaine]
└── orchestrator_[nom_du_sous_domaine].rs
```

## Isolement des sous-domaines

- Chaque sous-domaine doit être complètement isolé des autres sous-domaines
- Un sous-domaine ne peut pas appeler directement des méthodes d'un autre sous-domaine
- Toute communication entre sous-domaines passe exclusivement par l'ai_orchestrator.rs
- Chaque sous-domaine expose une API publique uniquement via son orchestrateur

## Délimitation claire des responsabilités

Pour éviter tout chevauchement entre les composants "low_level" et "managers", voici la délimitation précise:

### Low Level
- UNIQUEMENT les structures de données fondamentales et leurs opérations atomiques
- Pas de logique métier complexe
- Pas de connaissance du contexte d'utilisation
- Focus sur l'efficacité et la performance
- N'importe **aucun fichier externe au sous-domaine**
- N'appelle **aucune logique métier**
- Expose uniquement des **types**, des **algorithmes purs**, ou des **opérations atomiques**

### Managers
- Composition des opérations low_level pour former des fonctionnalités de plus haut niveau
- Toute la logique métier
- Connaissance du contexte et des règles d'utilisation
- Focus sur la cohérence et la correctitude

Exemple de délimitation:
- Low Level: "stocker un nœud avec ses propriétés"
- Manager: "créer un souvenir à partir d'une perception et l'intégrer dans le réseau de mémoire"

## Système d'apprentissage de l'IA

L'architecture intègre des capacités d'apprentissage centralisées et distribuées pour l'IA, organisées selon ces principes:

### Sous-domaine d'apprentissage

Un sous-domaine dédié `learning/` est responsable des mécanismes d'apprentissage centralisés:

```
learning/
├── low_level/
│   ├── models/             # Modèles fondamentaux d'apprentissage
│   ├── algorithms/         # Algorithmes d'optimisation et d'adaptation
│   └── metrics/            # Mesures d'efficacité et de performance
├── managers/
│   ├── feedback_manager.rs # Gestion des retours positifs/négatifs
│   ├── pattern_manager.rs  # Identification et consolidation de patterns
│   └── memory_manager.rs   # Optimisation et consolidation des mémoires
└── orchestrator_learning.rs
```

### Principes d'apprentissage distribué

1. **Apprentissage dual**: Chaque sous-domaine peut:
   - Gérer son propre apprentissage local via un manager dédié (`*_learning_manager.rs`)
   - Contribuer à l'apprentissage global en soumettant ses patterns au sous-domaine d'apprentissage

2. **Mécanismes d'amélioration continue**:
   - **Collecte passive**: Enregistrement automatique des comportements, réussites et échecs
   - **Amélioration active**: Processus explicites d'optimisation basés sur les données collectées
   - **Métaréflexion**: L'orchestrateur d'apprentissage peut analyser et améliorer le système global

3. **Flux du processus d'apprentissage**:
   ```
   Données → Feedback → Patterns → Optimisation → Consolidation → Nouvelles capacités
   ```

### Intégration avec l'orchestrateur principal

L'orchestrateur d'apprentissage expose à l'AI orchestrator principal:
- Des méthodes de collecte de données d'apprentissage
- Des processus d'optimisation à exécuter en arrière-plan
- Des processus de consolidation déclenchés par événements
- Des notifications de nouvelles capacités ou patterns découverts

### Interfaces d'apprentissage standardisées

Chaque sous-domaine implémentant des capacités d'apprentissage doit exposer:

1. Un trait `LearningCapable` standardisant:
   - La soumission de données d'apprentissage
   - L'application de nouveaux patterns identifiés
   - La mesure de l'efficacité des adaptations

2. Des métriques d'auto-évaluation permettant de guider l'apprentissage

Cette architecture garantit que l'IA peut non seulement accomplir ses tâches mais également s'améliorer progressivement en apprenant de ses propres expériences et interactions.

## Flux de données

```
                           +----------------+
                           | ai_orchestrator|
                           +----------------+
                              |         |
                 +------------+         +------------+
                 |                                   |
    +--------------------------+         +--------------------------+
    |orchestrator_subdomain_A  |         |orchestrator_subdomain_B  |
    +--------------------------+         +--------------------------+
             |         |                          |         |
    +-------------+  +-------------+     +-------------+  +-------------+
    | Manager A1  |  | Manager A2  |     | Manager B1  |  | Manager B2  |
    +-------------+  +-------------+     +-------------+  +-------------+
             |                |                |                |
    +-------------------+  +-------------------+  +-------------------+
    | Low Level Module A|  | Low Level Module B|  | Low Level Module C|
    +-------------------+  +-------------------+  +-------------------+
```

## Conventions de nommage et règles de développement

- Chaque orchestrateur doit implémenter le trait `Orchestrator` spécifique à son niveau
- Les noms des fichiers d'orchestrateurs suivent la convention `orchestrator_[nom_du_sous_domaine].rs`
- Les orchestrateurs de sous-domaine n'ont AUCUNE connaissance des autres sous-domaines
- Les managers ne doivent jamais communiquer directement entre eux, toujours via leur orchestrateur
- Le niveau bas (low_level) ne doit pas avoir de dépendances sur les managers
- Les orchestrateurs de sous-domaine exposent une API bien définie pour l'ai_orchestrator

## Contrôle de la visibilité

Pour garantir l'encapsulation de chaque sous-domaine et éviter tout accès non contrôlé :

- Le fichier `mod.rs` de chaque sous-domaine ne doit exposer **que** le fichier `orchestrator_[nom_du_sous_domaine].rs` via `pub mod`.
- Les dossiers `managers/` et `low_level/` ne doivent **jamais** être rendus publics dans le `mod.rs`.
- Tous les types définis dans `managers/` et `low_level/` doivent être `pub(crate)` ou privés. L'usage de `pub` est strictement réservé aux cas nécessitant une exposition au sein du domaine lui-même.
- Aucun composant interne ne doit être utilisé directement en dehors de son sous-domaine. L'orchestrateur local est **le seul point d'entrée autorisé** pour interagir avec les fonctionnalités du domaine.

Cette règle est essentielle pour assurer l'isolation, la testabilité et la maintenabilité des domaines métier.
