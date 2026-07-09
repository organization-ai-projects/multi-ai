# IDE Web - Multi AI

## Description

Ce projet est une interface web interactive pour travailler avec une intelligence artificielle (IA) intégrée. Il inclut des fonctionnalités telles que :
- Éditeur de code basé sur Monaco Editor.
- Suggestions en temps réel via WebSocket.
- Gestion des versions avec des actions comme le bump de version ou le revert.
- Exploration des fichiers disponibles dans le projet.
- Plugins IA pour enrichir les fonctionnalités de l'IDE.

## Fonctionnalités

1. **Éditeur de code** :
   - Chargement et sauvegarde des fichiers.
   - Support pour le langage Rust avec un thème sombre.

2. **Suggestions IA** :
   - Suggestions en temps réel via WebSocket.
   - Analyse des modifications pour proposer des actions comme le bump de version.

3. **Gestion des fichiers** :
   - Exploration des fichiers disponibles via l'endpoint `/files`.

4. **Gestion des versions** :
   - Bump de version automatique basé sur l'impact des modifications.
   - Revert vers des snapshots précédents.

5. **Plugins IA** :
   - **Auto-commenter les fonctions** : Ajoute des commentaires TODO pour chaque fonction détectée.
   - **Formatage automatique** : Indente automatiquement le code.
   - **Résumé de fichier** : Génère un résumé simple du fichier (nombre de lignes, fonctions, etc.).

6. **Chat IA** :
   - Ajout d'une barre latérale pour interagir avec l'IA en mode copilote.

## Nouveautés

1. **Support multi-fichiers** :
   - Gestion des onglets pour ouvrir plusieurs fichiers simultanément.
   - Sauvegarde et chargement indépendants pour chaque fichier.

2. **Support multi-langages** :
   - Langages supportés : Rust, JavaScript, Python, HTML, CSS.
   - Sélection dynamique du langage via un menu déroulant.

3. **Suggestions IA en temps réel** :
   - Ajout d'une route `/ws/trigger` pour déclencher une suggestion IA via WebSocket.
   - Bouton dans l'interface pour demander une suggestion IA sans recharger la page.

4. **Revert visuel par clic** :
   - Ajout d'une route `/revert/visual/:id` pour effectuer un revert visuel.
   - Bouton dans l'interface pour revenir rapidement à une version précédente.

5. **Plugins IA** :
   - Ajout d'un système de plugins pour enrichir les fonctionnalités de l'IDE :
     - Auto-commenter les fonctions.
     - Formatage automatique du code.
     - Résumé de fichier.

6. **Chat IA en barre latérale** :
   - Ajout d'une barre latérale pour interagir avec l'IA en mode copilote.

## Installation

1. Accédez au répertoire du crate `ide_web` :
   ```bash
   cd ide_web
   ```

2. Installez les dépendances Rust :
   ```bash
   cargo build
   ```

3. Lancez le serveur :
   ```bash
   cargo run
   ```

4. Accédez à l'interface web :
   - Ouvrez votre navigateur et allez sur [http://localhost:3000](http://localhost:3000).
   - Utilisez le bouton "💡 Demander une suggestion IA" pour tester les suggestions en temps réel.

## Endpoints

### API REST

- `GET /file/get/:path` : Récupère le contenu d'un fichier.
- `POST /file/save` : Sauvegarde le contenu d'un fichier.
- `GET /suggest` : Récupère une suggestion IA.
- `POST /bump` : Effectue un bump de version.
- `POST /revert/:id` : Revert vers un snapshot spécifique.
- `GET /files` : Liste les fichiers disponibles.
- `POST /ai/prompt` : Envoie un prompt à l'IA et récupère une réponse.

### WebSocket

- `GET /ws/suggestions` : Reçoit des suggestions IA en temps réel.
- `GET /ws/trigger` : Déclenche une suggestion IA via WebSocket.
- `POST /ws/contextual-suggestion/:file` : Envoie une suggestion contextuelle basée sur un fichier.

## Dépendances

- **Backend** :
  - [Axum](https://github.com/tokio-rs/axum) : Framework web pour Rust.
  - [Tokio](https://tokio.rs/) : Runtime asynchrone.
  - [Serde](https://serde.rs/) : Sérialisation et désérialisation.

- **Frontend** :
  - [Monaco Editor](https://microsoft.github.io/monaco-editor/) : Éditeur de code intégré.

## Structure du projet

```
ide_web/
├── public/         # Fichiers statiques (HTML, JS, CSS)
├── src/            # Code source du serveur
│   ├── routes.rs   # Définition des routes API
│   ├── controller.rs # Logique métier
│   ├── websocket.rs # Gestion des WebSockets
│   ├── filesystem.rs # Gestion des fichiers
│   ├── plugins.rs  # Système de plugins IA
│   ├── main.rs     # Point d'entrée du serveur
│   ├── lib.rs      # Configuration des modules
├── Cargo.toml      # Dépendances Rust
└── README.md       # Documentation
```

## Contribution

1. Forkez le projet.
2. Créez une branche pour vos modifications :
   ```bash
   git checkout -b feature/ma-nouvelle-fonctionnalite
   ```
3. Faites vos modifications et committez-les :
   ```bash
   git commit -m "Ajout d'une nouvelle fonctionnalité"
   ```
4. Poussez vos modifications :
   ```bash
   git push origin feature/ma-nouvelle-fonctionnalite
   ```
5. Ouvrez une Pull Request.

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus d'informations.
