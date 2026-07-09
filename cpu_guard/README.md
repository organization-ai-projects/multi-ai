# CPU Guard

`cpu_guard` est un outil de surveillance et de gestion des ressources système pour des processus d'intelligence artificielle (IA). Il surveille la température du CPU, l'utilisation du CPU, et prend des mesures correctives pour éviter la surchauffe ou la surcharge des ressources.

## Fonctionnalités principales

- **Surveillance de la température CPU** : Lecture des températures via des bibliothèques système ou des commandes spécifiques à la plateforme.
- **Surveillance de l'utilisation CPU** : Détection des surcharges CPU globales ou spécifiques à un processus.
- **Actions correctives** :
  - Mise en pause des processus en cas de surcharge.
  - Arrêt d'urgence en cas de température critique.
  - Reprise des processus après refroidissement.
- **Journalisation** : Enregistrement des événements au format binaire (pour traitement IA) et RON (pour lecture humaine).
- **Compatibilité multi-plateforme** : Support pour Linux et Windows.

## Prérequis

- **Rust** : Assurez-vous que Rust est installé sur votre machine. Vous pouvez l'installer via [rustup](https://rustup.rs/).
- **Dépendances système** :
  - Linux : Commandes `df`, `kill`, et `sh` doivent être disponibles.
  - Windows : PowerShell et WMI doivent être configurés.

## Installation

Accédez au dossier du projet et construisez-le avec Cargo :

```bash
cargo build -p cpu_guard
```

## Configuration

Le fichier de configuration des IA se trouve dans `shared_center/ia_list.ron`. Voici un exemple de configuration :

```ron
[
    (
        name: "IA_Example",
        manifest_path: "path/to/Cargo.toml",
        args: ["--example-arg"],
    ),
]
```

Vous pouvez également ajuster les paramètres de surveillance dans le fichier `config.rs` (par exemple, seuils de température et CPU).

## Exécution

Pour lancer la surveillance des IA, exécutez la commande suivante :

```bash
cargo run -p cpu_guard
```

Les logs seront enregistrés dans le dossier `shared_center/monitoring_logs`.

## Structure du projet

- **`src/main.rs`** : Point d'entrée principal.
- **`src/monitoring/`** : Modules pour la surveillance des ressources (température, CPU, etc.).
- **`src/process_control.rs`** : Gestion des processus (pause, reprise, arrêt).
- **`src/logging.rs`** : Gestion des logs.
- **`src/config.rs`** : Chargement et gestion des configurations.

## Journalisation

Les événements sont enregistrés dans deux formats :
- **Binaire** : Pour un traitement rapide par les IA.
- **RON** : Pour une lecture humaine.

Les fichiers de logs sont nommés avec un timestamp et se trouvent dans `shared_center/monitoring_logs`.

## Contribution

Les contributions sont les bienvenues ! Veuillez soumettre vos suggestions ou signaler des bugs.

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus de détails.
