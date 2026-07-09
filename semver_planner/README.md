# SemVer Planner

`semver_planner` est une bibliothèque Rust permettant de gérer les versions sémantiques (SemVer) en fonction des impacts détectés dans les snapshots.

## Fonctionnalités

- **Bump automatique** : Incrémente automatiquement la version en fonction de l'impact (Patch, Minor, Major).
- **Changelog** : Génère un changelog pour chaque nouvelle version.
- **Intégration avec Cargo.toml** : Met à jour automatiquement la version dans le fichier `Cargo.toml`.

## Nouveautés

- **Changelog détaillé** : Inclut les fichiers touchés et les informations de hash.

## Utilisation

Ajoutez cette bibliothèque comme dépendance dans votre projet Rust :

```toml
[dependencies]
semver_planner = { path = "../semver_planner" }
```

### Exemple

```rust
fn main() {
    let new_version = semver_planner::bump_from_snapshots();
    println!("📦 Nouvelle version : {}", new_version.to_string());
}
```

## Structure du projet

- `src/version.rs` : Gestion des versions SemVer.
- `src/planner.rs` : Logique principale pour incrémenter les versions et générer les changelogs.
- `src/loader.rs` : Chargement des impacts depuis les snapshots.

## Contribuer

Les contributions sont les bienvenues ! Veuillez soumettre une pull request ou ouvrir une issue pour discuter des améliorations.
