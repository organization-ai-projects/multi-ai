# AI Assistant

Ce projet implémente une IA assistant capable de gérer des versions de logiciels, d'analyser des changements, et de prendre des décisions basées sur un graphe mémoire.

## Fonctionnalités principales

### 1. Analyse des snapshots
- **Module** : `analyzer`
- Analyse les fichiers modifiés pour déterminer la zone dominante et l'impact des changements.

### 2. Classification de l'impact
- **Module** : `classifier`
- Classe les changements en trois catégories : `Patch`, `Minor`, et `Major`.

### 3. Génération de changelogs
- **Module** : `changelog`
- Génère un changelog basé sur la version, l'impact et les fichiers modifiés.

### 4. Validation des bumps de version
- **Module** : `validator`
- Valide si le bump de version est cohérent avec l'impact détecté.

### 5. Mémoire graphique
- **Module** : `graph_memory`
- Stocke les événements et leurs relations dans un graphe mémoire.
- Supporte les formats `.ron` et `.bin` pour la sauvegarde et le chargement.

### 6. Prise de décision et actions
- **Module** : `agent`
- Prend des décisions basées sur l'analyse et agit en conséquence (bump de version, etc.).
- Permet un arrêt propre via une commande `stop`.

### 7. Interface CLI
- **Module** : `cli`
- Fournit une interface en ligne de commande pour interagir avec l'IA.
- Commandes disponibles :
  - `suggest <path>` : Analyse un projet et propose des actions.
  - `log` : Affiche le journal des décisions.
  - `memory show` : Affiche la mémoire actuelle.
  - `decision explain <id>` : Explique une décision spécifique.
  - `feedback <id> <good/bad>` : Ajoute un feedback utilisateur à une décision.

### 8. Rapport de session
- Génère un fichier `.graphver/ai_activity.md` contenant un résumé des décisions prises et des anomalies détectées.

## Utilisation

### Démarrage
1. Compilez le projet avec `cargo build`.
2. Lancez l'IA avec `cargo run`.
3. Utilisez les commandes CLI pour interagir avec l'IA.

### Sauvegarde de la mémoire
- La mémoire est automatiquement sauvegardée dans deux formats :
  - **Fichier `.ron`** : lisible et éditable manuellement.
  - **Fichier `.bin`** : format binaire optimisé pour l'IA.

### Chargement de la mémoire
- Chargez un graphe mémoire existant avec :
  ```rust
  let graph = AiGraph::load_from_file("graph_memory.ron").unwrap();
  // ou
  let graph = AiGraph::load_from_bin("graph_memory.bin").unwrap();
  ```

## Dépendances
- **[serde](https://crates.io/crates/serde)** : Sérialisation et désérialisation.
- **[ron](https://crates.io/crates/ron)** : Format de sérialisation lisible.
- **[bincode](https://crates.io/crates/bincode)** : Sérialisation binaire rapide.
- **[clap](https://crates.io/crates/clap)** : Parsing des arguments CLI.
- **[chrono](https://crates.io/crates/chrono)** : Gestion des dates pour les rapports.

## Structure du projet

```
src/
├── actions.rs       # Gestion des actions (bump, revert)
├── agent.rs         # Prise de décision et exécution
├── analyzer.rs      # Analyse des snapshots
├── changelog.rs     # Génération de changelogs
├── classifier.rs    # Classification de l'impact
├── cli.rs           # Interface en ligne de commande
├── graph_memory.rs  # Gestion du graphe mémoire
├── lib.rs           # Point d'entrée des modules
├── main.rs          # Point d'entrée principal
├── validator.rs     # Validation des bumps de version
```

## Commandes utiles

- **Démarrer l'IA** : `cargo run`
- **Arrêter l'IA** : Tapez `stop` dans la console.
- **Utiliser le CLI** : `cargo run -- <commande>`
- **Compiler** : `cargo build`
- **Exécuter les tests** : `cargo test`

## Contribuer
1. Forkez le projet.
2. Créez une branche pour vos modifications : `git checkout -b feature/ma-fonctionnalite`.
3. Faites un commit : `git commit -m "Ajout de ma fonctionnalité"`.
4. Poussez vos modifications : `git push origin feature/ma-fonctionnalite`.
5. Ouvrez une Pull Request.

## Licence
Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.
