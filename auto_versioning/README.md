# Auto Versioning

Un outil intelligent pour automatiser le versioning sémantique des projets Rust.

## 🎯 Fonctionnalités

- 🔄 Surveillance en temps réel des changements de code
- 📊 Détection automatique de l'impact des modifications (major/minor/patch)
- 📝 Génération automatique du CHANGELOG.md
- 🚀 Publication automatisée (crates.io, GitHub, VSCode)
- 🔙 Système de rollback intégré
- 🌐 API REST pour contrôle externe
- 📈 Visualisation des dépendances
- 🤝 Multi-format de configuration (RON, TOML)

## 🚀 Installation

```bash
cargo install auto_versioning
```

## 📖 Usage

```bash
# Surveillance simple
auto_versioning <project_path>

# Options avancées
auto_versioning <project_path> [options]
  --force-publish      Force la publication
  --dry-run           Simulation sans modifications
  --bump=<version>    Version manuelle
  --publish=<target>  [crates|gh|both]
  --no-tag           Skip les tags Git
  --test-rules       Test des règles
  --discord-webhook  URL webhook Discord
  --github-webhook   URL webhook GitHub
  --log=<level>      Niveau de log [debug|info|warn]
```

## 🌟 Nouvelles Fonctionnalités

- 🔄 Support multi-projet (workspaces Rust)
- 📊 Graphe de dépendances visualisable
- 🔙 Snapshots et rollback system
- 🤖 Commandes post-publication configurables
- 📡 Webhooks Discord/GitHub
- 🎯 Cache par projet pour performance
- 📝 Auto-commit des changements
- 🎨 Interface web de visualisation

## ⚙️ Configuration

### project.avconfig.ron
```ron
(
    patterns: [
        (
            match_str: "pub trait",
            impact: "major",
            description: "API publique"
        ),
    ],
    exclude: ["target/*"],
    post_publish: [
        (
            cmd: "cargo",
            args: ["doc"],
            condition: "on_success"
        ),
    ],
    authors: [
        (
            github: "organization/team-core",
            cargo: "cargo-team-name"
        ),
    ],
    cache_dir: ".av-cache",
    publish_targets: ["Crates", "GitHub"],
)
```

### patterns.ron
```ron
(
    weights: {
        "pub trait": "major",
        "pub fn": "minor",
        "fix:": "patch",
    }
)
```

## 🔌 API REST

```bash
# Health check
curl http://localhost:3030/health

# Déclencher une vérification
curl -X POST http://localhost:3030/trigger \
  -H "Content-Type: application/json" \
  -d '{"project_path": "./my_project", "force": true}'

# Stream des updates
curl http://localhost:3030/updates
```

## 🏗️ Architecture

```
src/
├── main.rs           # Point d'entrée
├── analyzer.rs       # Analyse d'impact
├── watcher.rs        # Surveillance fichiers
├── backup.rs         # Système de backup
├── changelog.rs      # Gestion CHANGELOG
├── cli.rs           # Interface CLI
├── config.rs        # Configuration
├── git.rs           # Intégration Git
├── graph.rs         # Graphe dépendances
├── publishing.rs     # Publication multi-cible
├── webhook.rs       # Notifications
├── snapshot.rs      # Système de backup
├── cache.rs         # Cache par projet
├── workspace.rs     # Support multi-projet
└── commands.rs      # Post-commandes
```

## 🛠️ Développement

```bash
# Tests
cargo test

# Tests des règles
cargo run -- --test-rules

# Build en mode release
cargo build --release
```

## 🤝 Contribution

1. Fork le projet
2. Créer une branche (`git checkout -b feature/amazing`)
3. Commit les changements (`git commit -am 'Add feature'`)
4. Push la branche (`git push origin feature/amazing`)
5. Ouvrir une Pull Request

## 📄 Licence

MIT License

## 🧠 Système d'Intelligence

Le système utilise plusieurs niveaux d'intelligence :

### Apprentissage par Pattern
- Détection des patterns de code impactants
- Adaptation à la codebase du projet
- Mémorisation des succès/erreurs

### Arbre de Décision
```rust
// Classification des changements via linfa-trees
let tree = DecisionTree::params()
    .max_depth(5)
    .min_samples_split(2)
    .fit(&dataset)?;
```

### Features Analysées
- Patterns syntaxiques (pub trait, breaking changes...)
- Contexte du fichier (api/, core/, tests/...)
- Historique des impacts
- Dépendances entre fichiers

### Auto-Amélioration
- Apprentissage des erreurs de prédiction
- Ajustement des poids par contexte
- Cache des patterns fiables
- Feedback continu

### Configuration
```ron
// .av-brain.ron - Configuration du système intelligent
(
    history_size: 100,     // Taille de l'historique conservé
    confidence_threshold: 0.8,  // Seuil de confiance minimum
    learning_rate: 0.1,    // Vitesse d'apprentissage
)
```

Pour plus de détails, voir [INTELLIGENCE.md](docs/INTELLIGENCE.md).
