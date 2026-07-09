# Architecture du Bridge

## Vue d'ensemble

Le bridge est une couche d'abstraction qui sépare clairement les responsabilités entre :
- L'interface web (IDE)
- Le système d'IA
- Le système de versioning

```
IDE Web <-> Bridge <-> (IA + Versioning)
```

## Structure

```
bridge/
├── mod.rs           # Exports des modules
├── ai_bridge.rs     # Pont vers l'IA
├── version_bridge.rs # Pont vers le versioning
└── smart_bridge.rs  # Combinaison intelligente IA+Versioning
```

## Rôles des composants

### 1. ai_bridge.rs
Responsable de toutes les interactions avec l'IA :
- Suggestions de code
- Analyse des changements
- Accès à la mémoire d'apprentissage
- Point d'entrée unique vers `ai_assistant`
- Transforme les réponses de l'IA en format web

```rust
// Exemple d'utilisation
let suggestion = ai_bridge::get_suggestion("src/lib.rs");
let impact = ai_bridge::analyze_changes(&files);
```

### 2. version_bridge.rs
Gère tout ce qui concerne le versioning :
- Bump de versions
- Reverts
- Gestion du graphe de versions
- Point d'entrée unique vers `version_watcher`
- Convertit les données pour l'interface web

```rust
// Exemple d'utilisation
version_bridge::bump()?;
version_bridge::revert("abc123")?;
```

### 3. smart_bridge.rs
Combine l'IA et le versioning pour créer des fonctionnalités avancées :
- Bump intelligent avec analyse IA
- Revert interactif avec vérifications IA
- Suggestions de stratégies de versioning
- Utilise uniquement `version_bridge` et `ai_bridge`
- Ne communique jamais directement avec les crates externes

```rust
// Exemple d'utilisation
smart_bridge::smart_version_bump()?;
smart_bridge::suggest_version_strategy()?;
```

## Flux de données

1. **Entrée depuis l'IDE** :
   ```
   IDE -> routes.rs -> controller.rs -> bridge -> systèmes sous-jacents
   ```

2. **Retour vers l'IDE** :
   ```
   systèmes sous-jacents -> bridge (conversion JSON) -> controller -> routes -> IDE
   ```

## Diagramme des dépendances détaillé

```
IDE Web -> bridge/
           ├── version_bridge.rs -> version_watcher
           │   - Gère les bump de versions
           │   - Gère les reverts
           │   - Fournit le graphe de versions
           │
           ├── ai_bridge.rs ------> ai_assistant
           │   - Fournit des suggestions IA
           │   - Analyse les changements
           │   - Accède à la mémoire d'apprentissage
           │
           └── smart_bridge.rs ----> (version_bridge + ai_bridge)
               - Combine les fonctionnalités IA et versioning
               - Fournit des stratégies avancées
               - Gère les reverts interactifs
```

## Principes de design

1. **Séparation des responsabilités** :
   - Chaque bridge a un rôle unique et clair
   - Pas de duplication de logique entre les bridges

2. **Format des données** :
   - Entrée : Types Rust natifs
   - Sortie : JSON pour compatibilité web

3. **Gestion des erreurs** :
   - Utilisation de `Result` pour la propagation d'erreurs
   - Messages d'erreur clairs et contextuels

4. **Performance** :
   - Format binaire pour les opérations fréquentes
   - JSON pour la compatibilité et la lisibilité

## Exemple d'utilisation complet

```rust
// Dans controller.rs
use crate::bridge::{ai_bridge, version_bridge, smart_bridge};

pub async fn handle_version_bump() -> Result<String, Error> {
    // 1. Analyse IA des changements
    let files = get_modified_files();
    let impact = ai_bridge::analyze_changes(&files);

    // 2. Si changement majeur, utiliser le smart bridge
    if impact.get("level") == Some("major") {
        smart_bridge::smart_version_bump()
    } else {
        // 3. Sinon, utiliser le version bridge standard
        version_bridge::bump()
    }
}
```

## Extensions futures

- Ajout de nouveaux bridges pour d'autres systèmes
- Enrichissement des capacités de smart_bridge
- Amélioration des performances avec du caching
