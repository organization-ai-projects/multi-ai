# Architecture de la Mémoire Graphique

Ce document décrit l'architecture du système de mémoire graphique utilisé dans le projet Mother AI, avec un focus sur la séparation des responsabilités et les relations entre les différents composants.

## Vue d'ensemble de l'architecture

L'architecture suit un modèle en couches avec une séparation claire des responsabilités :

```
+---------------------------+
|     PublicMemory          |  <-- Couche d'interface publique
+---------------------------+
           |
           v
+---------------------------+
|  StorageGraphMemory       |  <-- Couche de persistance
+---------------------------+
           |
           v
+---------------------------+
|  GraphMemoryManager       |  <-- Couche d'orchestration
+---------------------------+
         /     \
        /       \
       v         v
+----------+ +------------+
|NodesManager| |LinksManager|  <-- Couche de gestion spécialisée
+----------+ +------------+
      |             |
      v             v
+--------+     +--------+
|  Node  |     |  Link  |  <-- Couche de données (entités)
+--------+     +--------+
```

## Composants et leurs responsabilités

### 0. Couche d'interface publique

#### `PublicMemory` (`public_graph_memory.rs`)
- **Responsabilité** : Exposer une API simplifiée pour les autres modules du système
- **Relations** :
  - Utilise `MemoryStorage` (qui est probablement un wrapper autour de `StorageGraphMemory`)
  - Exposé aux autres modules/crates du système
- **Interfaces** :
  - Chargement de la mémoire d'une IA par son nom
  - Sauvegarde de la mémoire d'une IA
  - Suppression des fichiers de mémoire
- **Limites** :
  - Interface très simplifiée (retours booléens)
  - Pas d'accès direct aux structures de données de la mémoire

### 1. Couche de données (Entités)

#### `Node` (`node.rs`)
- **Responsabilité** : Représentation d'un nœud dans le graphe de mémoire
- **Interfaces** :
  - Création et manipulation d'un nœud
  - Gestion des attributs et tags
- **Limites** : 
  - Visibilité `pub(crate)` - accessible seulement dans le crate
  - Pas d'accès direct depuis l'extérieur du module mémoire

#### `Link` (`link.rs`)
- **Responsabilité** : Représentation d'un lien entre deux nœuds
- **Interfaces** :
  - Création et manipulation d'un lien
  - Propriétés comme source, cible, label et poids
- **Limites** :
  - Visibilité `pub(crate)` - accessible seulement dans le crate
  - Représente une relation directionnelle d'un nœud à un autre

### 2. Couche de gestion spécialisée

#### `NodesManager` (`nodes_manager.rs`)
- **Responsabilité** : Gestion d'une collection de nœuds
- **Relations** :
  - Utilise directement `Node`
  - Utilisé par `GraphMemoryManager`
- **Interfaces** :
  - Création et ajout de nœuds
  - Construction de nœuds incrémentale (pattern builder)
  - Recherche et interrogation de nœuds
  - Manipulation des attributs et tags
- **Limites** :
  - Ne gère que les nœuds, pas leurs relations
  - N'a pas de connaissance du stockage persistant

#### `LinksManager` (`links_manager.rs`)
- **Responsabilité** : Gestion d'une collection de liens
- **Relations** :
  - Utilise directement `Link`
  - Utilisé par `GraphMemoryManager`
- **Interfaces** :
  - Création et ajout de liens
  - Filtrage et recherche de liens
  - Désactivation de liens
- **Limites** :
  - Ne vérifie pas l'existence des nœuds référencés
  - N'a pas de connaissance du stockage persistant

### 3. Couche d'orchestration

#### `GraphMemoryManager` (`manager_graph_memory.rs`)
- **Responsabilité** : Coordination des opérations sur l'ensemble du graphe
- **Relations** :
  - Utilise `NodesManager` et `LinksManager`
  - Utilisé par `StorageGraphMemory`
- **Interfaces** :
  - Délégation aux managers spécialisés
  - Opérations impliquant à la fois des nœuds et des liens
  - Vérification de cohérence (ex: nœuds existants avant de créer un lien)
- **Limites** :
  - Ne gère pas la persistance
  - Exposé publiquement, mais n'expose pas directement les structures internes

### 4. Couche de persistance

#### `StorageGraphMemory` (`storage_graph_memory.rs`)
- **Responsabilité** : Sauvegarde et chargement de la mémoire graphique
- **Relations** :
  - Utilise `GraphMemoryManager`
  - Utilise `PathManager` pour localiser les fichiers
- **Interfaces** :
  - Sauvegarde aux formats RON et binaire
  - Chargement avec priorité au format binaire
- **Limites** :
  - Statique (non-instanciable)
  - Dépend de la sérialisation de `GraphMemoryManager`

## Flux de travail typiques

### Utilisation de l'interface publique
1. Un module externe utilise `PublicMemory` pour charger/sauvegarder la mémoire d'une IA
2. `PublicMemory` délègue l'opération à `MemoryStorage`
3. Le module n'a pas besoin de connaître les détails d'implémentation de la mémoire

### Création et manipulation de nœuds
1. L'application obtient une instance de `GraphMemoryManager`
2. Elle appelle `create_and_add_node()` ou utilise le pattern builder avec `create_node_begin()`/`create_node_add_attribute()`/`create_node_finish()`
3. `GraphMemoryManager` délègue au `NodesManager`
4. `NodesManager` utilise `Node` pour créer et stocker le nœud

### Création de liens entre nœuds
1. L'application appelle `create_and_add_link()` sur `GraphMemoryManager`
2. `GraphMemoryManager` vérifie l'existence des nœuds via `NodesManager`
3. Si les nœuds existent, `GraphMemoryManager` délègue la création au `LinksManager`

### Persistance de la mémoire
1. L'application appelle `StorageGraphMemory::save()` avec une instance de `GraphMemoryManager`
2. `StorageGraphMemory` sérialise l'instance en RON et binaire
3. Pour charger, l'application appelle `StorageGraphMemory::load()`

## Avantages de cette architecture

1. **Séparation des responsabilités** : Chaque composant a un rôle unique et bien défini
2. **Encapsulation** : Les détails d'implémentation sont cachés derrière des interfaces claires
3. **Réduction du couplage** : Les dépendances sont minimisées et unidirectionnelles
4. **Flexibilité** : Les composants peuvent évoluer indépendamment
5. **Testabilité** : Chaque couche peut être testée séparément
6. **Extensibilité** : Facile d'ajouter de nouvelles fonctionnalités sans modifier l'existant
7. **Sécurité d'API** : `PublicMemory` offre une interface minimaliste qui cache les détails d'implémentation

## Limites et points d'amélioration

1. **Performance** : Les multiples couches peuvent introduire une légère surcharge
2. **Complexité initiale** : Plus de fichiers et de concepts à comprendre
3. **Transactions** : Pas de support natif pour les opérations atomiques
4. **Concurrence** : L'architecture actuelle n'est pas optimisée pour l'accès concurrent
5. **Observabilité** : Pourrait bénéficier d'un système d'événements ou d'observateurs
6. **Granularité de l'API publique** : L'interface `PublicMemory` est actuellement limitée aux opérations de base, sans accès aux fonctionnalités avancées

## Évolutions possibles

1. **API asynchrone** pour les opérations de persistance
2. **Système d'événements** pour notifier des changements dans le graphe
3. **Versionnement de la mémoire** pour suivre l'évolution du graphe
4. **Indexation avancée** pour accélérer certaines requêtes
5. **Intégration de bases de données graphes** pour les grands volumes de données
6. **Enrichissement de l'API publique** pour exposer plus de fonctionnalités de manière contrôlée
7. **Gestion des erreurs** plus précise dans l'interface publique (au lieu de simples booléens)
