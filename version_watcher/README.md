# Version Watcher

`version_watcher` est un outil Rust permettant de surveiller les modifications dans un projet et de créer des snapshots versionnés.

## Fonctionnalités

- **Surveillance des fichiers** : Détecte les modifications, créations et suppressions dans un répertoire.
- **Snapshots** : Crée des snapshots versionnés avec un hash unique, un timestamp et un impact estimé.
- **Graphe de versions** : Maintient un graphe des versions pour suivre l'historique des modifications.
- **Revert** : Permet de revenir à une version précédente.
- **Multi-repo** : Fusionne plusieurs graphes dans un graphe global.

## Nouveautés

- **CLI complète** : Commandes `watch`, `graph log`, `graph latest`, `graph revert`.
- **Fusion multi-repo** : Scanne et fusionne les graphes de plusieurs projets.

## Utilisation

Ajoutez cette bibliothèque comme dépendance dans votre projet Rust :

```toml
[dependencies]
version_watcher = { path = "../version_watcher" }
```

### Exemple

```bash
# Surveiller un répertoire
version-watcher watch ./mon-projet

# Afficher le log du graphe
version-watcher graph log

# Revenir à une version précédente
version-watcher graph revert <id>

# Fusionner plusieurs graphes
version-watcher graph consolidate ./repo-a/.graphver ./repo-b/.graphver
```

## Structure du projet

- `src/watcher.rs` : Surveillance des fichiers et détection des changements.
- `src/store.rs` : Gestion des snapshots et du graphe de versions.
- `src/graph.rs` : Manipulation du graphe de versions.
- `src/cli.rs` : Interface en ligne de commande.

## Contribuer

Les contributions sont les bienvenues ! Veuillez soumettre une pull request ou ouvrir une issue pour discuter des améliorations.
