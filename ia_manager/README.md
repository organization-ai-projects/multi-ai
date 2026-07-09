# IA Manager

`ia_manager` est un outil de gestion pour les projets d'intelligence artificielle (IA) dans un workspace Rust. Il permet de détecter automatiquement les projets d'IA, de générer des configurations, et de préparer les environnements nécessaires pour leur exécution.

## Fonctionnalités principales

- **Détection automatique des projets d'IA** : Scanne le workspace pour identifier les projets d'IA basés sur leur structure.
- **Génération de configuration** : Crée un fichier `ia_list.ron` contenant les informations nécessaires pour chaque IA.
- **Préparation des environnements** :
  - Création des répertoires de mémoire et de sandbox pour chaque IA.
  - Initialisation des fichiers de mémoire (`memory.bin` et `memory.ron`).

## Prérequis

- **Rust** : Assurez-vous que Rust est installé sur votre machine. Vous pouvez l'installer via [rustup](https://rustup.rs/).

## Installation

Accédez au dossier du projet et construisez-le avec Cargo :

```bash
cargo build -p ia_manager
```

## Utilisation

Pour exécuter `ia_manager` et générer les configurations et environnements nécessaires, utilisez la commande suivante :

```bash
cargo run -p ia_manager
```

## Structure du projet

- **`src/main.rs`** : Point d'entrée principal.
- **Détection des projets** : Recherche des projets d'IA dans le dossier `ai/`.
- **Génération de configuration** : Crée le fichier `shared_center/ia_list.ron`.
- **Préparation des environnements** : Crée les répertoires et initialise les fichiers nécessaires pour chaque IA.

## Configuration générée

Le fichier `ia_list.ron` contient les informations suivantes pour chaque IA détectée :

```ron
[
    (
        name: "IA_Example",
        memory_path: "memory/IA_Example",
        sandbox_path: "sandbox/IA_Example",
        manifest_path: "ai/IA_Example/Cargo.toml",
        args: ["--verbose"],
    ),
]
```

## Contribution

Les contributions sont les bienvenues ! Veuillez soumettre vos suggestions ou signaler des bugs.

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus de détails.
