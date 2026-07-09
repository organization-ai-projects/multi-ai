# Emerging AI

## Description
Emerging AI est un projet visant à créer une intelligence artificielle capable d'apprendre et de muter du code Rust. Le projet utilise un graphe mémoire pour stocker les snippets de code, leurs mutations, et les résultats des compilations.

---

## Fonctionnalités existantes

### 1. **Extraction de snippets de code**
- Extraction des fonctions Rust depuis des fichiers `.rs` dans un dossier donné (`sandbox` dans le projet actuel).
- Utilisation de la bibliothèque `syn` pour parser le code.

### 2. **Mutation de code**
- Mutation des fonctions extraites en modifiant l'ordre des instructions dans le corps des fonctions.
- Utilisation de la bibliothèque `rand` pour générer des mutations aléatoires.

### 3. **Compilation et test**
- Compilation des snippets mutés dans un environnement temporaire (`sandbox`).
- Utilisation de `cargo check` pour vérifier la validité des mutations.
- Utilisation de `cargo run` pour tester l'exécution des mutations.
- Ajout d'un système de scoring :
  - **+3 points** : Compilation et exécution réussies.
  - **+1 point** : Compilation réussie, mais échec d'exécution.
  - **-1 point** : Échec de compilation.

### 4. **Graphe mémoire**
- Stockage des snippets originaux et mutés dans un graphe mémoire.
- Suivi des succès et échecs des mutations.
- Sauvegarde et chargement de la mémoire en formats binaires (`.bin`) et RON (`.ron`).
- Chargement par défaut depuis le fichier binaire (`memory.bin`) pour des performances optimales.

### 5. **Boucle d’apprentissage**
- Sélection aléatoire de snippets pour mutation.
- Feedback sur les mutations réussies ou échouées pour ajuster le graphe mémoire.

---

## Ce qui manque

### 1. **Amélioration des mutations**
- Ajouter des mutations plus complexes (ex. : modification des types, ajout de nouvelles instructions, etc.).
- Permettre à l'IA de découvrir ses propres stratégies de mutation.

### 2. **Analyse des résultats**
- Générer des rapports détaillés sur les mutations réussies et échouées.
- Visualiser le graphe mémoire pour mieux comprendre les relations entre les snippets.

### 3. **Optimisation des performances**
- Réduire le temps de compilation des snippets.
- Optimiser la gestion de la mémoire pour les grands graphes.

### 4. **Tests unitaires**
- Ajouter des tests unitaires pour chaque module (`parser`, `mutate_ast`, `try_compilation`, etc.).
- Vérifier la robustesse des mutations et des compilations.

### 5. **Documentation**
- Ajouter des commentaires détaillés dans le code.
- Documenter les API publiques pour faciliter l'extension du projet.

---

## Structure du projet

```
emerging_ai/
├── src/
│   ├── brain/
│   │   ├── memory.rs         # Gestion du graphe mémoire
│   │   ├── persistent.rs     # Sauvegarde et chargement de la mémoire
│   │   └── mod.rs            # Module brain
│   ├── capacities/
│   │   ├── mutate_ast.rs     # Mutations des fonctions
│   │   ├── parser.rs         # Extraction des fonctions
│   │   ├── try_compilation.rs # Compilation, test et scoring
│   │   └── mod.rs            # Module capacities
│   └── main.rs               # Point d'entrée principal
├── sandbox/                  # Dossier contenant les fichiers Rust pour l'apprentissage
├── mutations_tmp/            # Dossier temporaire pour les mutations
├── Cargo.toml                # Dépendances et configuration
└── README.md                 # Documentation du projet
```

---

## Instructions pour démarrer

1. **Installer les dépendances**
   ```bash
   cargo build
   ```

2. **Exécuter le projet**
   ```bash
   cargo run
   ```

3. **Ajouter des snippets pour l'apprentissage**
   - Placez vos fichiers `.rs` dans le dossier `sandbox` à la racine du projet.

4. **Analyser les résultats**
   - Les fichiers `memory.bin` et `memory.ron` contiennent l'état du graphe mémoire.
   - Le scoring est intégré pour évaluer la qualité des mutations.

---

## Contributions
Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou soumettre une pull request pour proposer des améliorations.
