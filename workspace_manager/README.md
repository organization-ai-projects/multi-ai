# Workspace Manager

## Description

`workspace_manager` est le système central qui orchestre les différents composants du projet `multi_ai`. Il agit comme un panneau de contrôle global, permettant de gérer les services (comme `ide_web`), les configurations, et les projets multiples.

## Fonctionnalités principales

1. **Gestion des services** :
   - Démarrage et arrêt des services comme `ide_web`.
   - API pour interagir avec les services via HTTP.

2. **Gestion des projets multiples** :
   - Détection et gestion des projets dans un environnement multi-projets.
   - Génération de fichiers de configuration et de logs.

3. **Initialisation automatique** :
   - Création de la structure du projet si elle n'existe pas.
   - Activation des composants nécessaires en fonction du mode (`ide`, `intelligent`, etc.).

4. **Interface utilisateur (UI)** :
   - Une UI web simple pour gérer les services et afficher les informations du projet.

## Structure du projet

```
workspace_manager/
├── src/
│   ├── config.rs       # Gestion de la configuration du projet
│   ├── context.rs      # Contexte du projet (nom, mode, etc.)
│   ├── http.rs         # Serveur HTTP et API
│   ├── init.rs         # Initialisation de la structure du projet
│   ├── paths.rs        # Gestion des chemins et des fichiers
│   ├── services.rs     # Gestion des services (démarrage/arrêt)
│   ├── workspace.rs    # Vue complète du projet courant
│   ├── lib.rs          # Point d'entrée des modules
├── public/             # Fichiers statiques pour l'UI web
│   ├── index.html      # Interface utilisateur principale
├── Cargo.toml          # Dépendances Rust
└── README.md           # Documentation
```

## Fonctionnement

### 1. Initialisation du projet

Lors du premier démarrage, `workspace_manager` :
- Crée la structure du projet (dossiers `.ide`, `.intelli`, etc.).
- Génère un fichier de configuration par défaut (`project_config.ron`).
- Active automatiquement les composants nécessaires en fonction du mode.

### 2. Gestion des services

`workspace_manager` permet de démarrer et d'arrêter des services comme `ide_web` via des API HTTP ou l'interface utilisateur.

#### API pour les services

- **Lister les services** :
  ```http
  GET /api/services
  ```
  Réponse :
  ```json
  [
    { "name": "ide_web", "running": true },
    { "name": "ai_assistant", "running": false }
  ]
  ```

- **Démarrer un service** :
  ```http
  POST /api/service/start/:name
  ```
  Exemple :
  ```bash
  curl -X POST http://localhost:4000/api/service/start/ide_web
  ```

- **Arrêter un service** :
  ```http
  POST /api/service/stop/:name
  ```
  Exemple :
  ```bash
  curl -X POST http://localhost:4000/api/service/stop/ide_web
  ```

### 3. Interface utilisateur

L'interface utilisateur est accessible via un navigateur web à l'adresse suivante :
```
http://localhost:4000
```

Elle permet de :
- Lister les services disponibles.
- Démarrer ou arrêter les services.
- Afficher les informations du projet.

### 4. Gestion des projets multiples

`workspace_manager` détecte automatiquement les projets dans un environnement multi-projets et génère un fichier de log contenant les informations suivantes :
- Nom du projet.
- Mode (`ide`, `intelligent`, etc.).
- Liste des composants actifs.

## Installation

1. Clonez le dépôt :
   ```bash
   git clone https://github.com/votre-utilisateur/multi_ai.git
   cd multi_ai/workspace_manager
   ```

2. Installez les dépendances :
   ```bash
   cargo build
   ```

3. Lancez le serveur :
   ```bash
   cargo run
   ```

4. Accédez à l'interface utilisateur :
   - Ouvrez votre navigateur et allez sur [http://localhost:4000](http://localhost:4000).

## Dépendances

- **Backend** :
  - [Axum](https://github.com/tokio-rs/axum) : Framework web pour Rust.
  - [Tokio](https://tokio.rs/) : Runtime asynchrone.
  - [Serde](https://serde.rs/) : Sérialisation et désérialisation.
  - [Tower HTTP](https://github.com/tower-rs/tower-http) : Middleware HTTP.

## Extensions futures

- Ajout d'une gestion avancée des logs.
- Intégration avec des services externes.
- Support pour des configurations personnalisées par projet.

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus d'informations.
