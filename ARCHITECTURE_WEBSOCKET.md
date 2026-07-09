# Architecture Globale du Système WebSocket

## Vue d'ensemble

Le système WebSocket est conçu pour permettre une communication en temps réel entre les différents composants du projet `multi_ai`. Chaque projet peut fonctionner de manière isolée, sauf `semver_planner`, qui dépend à la fois de `ai_assistant` et de `version_watcher`. 

`workspace_manager` agit comme un orchestrateur indépendant, permettant d'activer ou de désactiver les projets selon les besoins.

```
(ai_assistant) <-> (semver_planner) <-> (version_watcher)
       ^
       |
(ide_web)
       ^
       |
(workspace_manager)
```

---

## Composants principaux

### 1. `ai_assistant` (Projet central)
- **Rôle** : Fournir des fonctionnalités d'intelligence artificielle.
- **Fonctionnalités** :
  - Suggestions de code.
  - Analyse des changements.
  - Prédiction des stratégies de version.
- **Indépendance** : Peut fonctionner seul ou être intégré à d'autres projets.

### 2. `semver_planner`
- **Rôle** : Gestion des versions sémantiques (SemVer).
- **Fonctionnalités** :
  - Bump automatique des versions.
  - Génération de changelogs.
  - Mise à jour automatique du fichier `Cargo.toml`.
- **Dépendances** :
  - Dépend de `ai_assistant` pour valider les impacts des changements.
  - Dépend de `version_watcher` pour accéder aux snapshots et à l'historique des versions.

### 3. `version_watcher`
- **Rôle** : Gestion des versions et des snapshots.
- **Fonctionnalités** :
  - Création de snapshots versionnés.
  - Revert vers des versions précédentes.
  - Analyse des impacts des modifications.
- **Indépendance** : Peut fonctionner seul ou être intégré à d'autres projets.

### 4. `ide_web`
- **Rôle** : Interface utilisateur pour interagir avec les services.
- **Fonctionnalités** :
  - Éditeur de code avec suggestions IA.
  - Gestion des fichiers et des versions.
  - Plugins pour enrichir les fonctionnalités (auto-commentaire, formatage, résumé).
- **Dépendances** :
  - Dépend de `ai_assistant` pour les suggestions IA.
  - Dépend de `version_watcher` pour la gestion des versions.
  - Dépend de `semver_planner` pour les bump de versions et les changelogs.

### 5. `workspace_manager` (Orchestrateur)
- **Rôle** : Hub central pour gérer les services et router les messages WebSocket.
- **Fonctionnalités** :
  - Démarrage et arrêt des services (`ide_web`, `ai_assistant`, `version_watcher`, `semver_planner`, etc.).
  - Routage des messages WebSocket vers les services appropriés.
- **Indépendance** : Ne dépend d'aucun autre projet.

---

## Flux de données

### 1. Communication entre les composants
- Chaque composant expose un serveur WebSocket pour ses fonctionnalités.
- `workspace_manager` agit comme un intermédiaire pour router les messages entre les composants.

### 2. Exemple de flux
1. **IDE Web demande une suggestion IA** :
   - Envoie un message WebSocket à `workspace_manager` avec le préfixe `ai:`.
   - `workspace_manager` route le message vers `ai_assistant`.
   - `ai_assistant` traite la commande et renvoie une réponse via WebSocket.

2. **IDE Web effectue un bump de version** :
   - Envoie un message WebSocket à `workspace_manager` avec le préfixe `version:`.
   - `workspace_manager` route le message vers `version_watcher`.
   - `version_watcher` effectue le bump et renvoie une confirmation.

3. **Validation d'un bump de version** :
   - `semver_planner` utilise `ai_assistant` pour valider l'impact du bump.
   - `semver_planner` génère un changelog basé sur les informations de `version_watcher`.

---

## Indépendance et intégration

### Indépendance
- **`ai_assistant`** et **`version_watcher`** peuvent fonctionner seuls.
- **`semver_planner`** nécessite `ai_assistant` et `version_watcher` pour fonctionner.
- **`ide_web`** nécessite les trois (`ai_assistant`, `version_watcher`, et `semver_planner`).

### Intégration via `workspace_manager`
- `workspace_manager` permet d'activer ou de désactiver les projets selon les besoins.
- Les projets communiquent via WebSocket, évitant les dépendances directes.

---

## Avantages de l'architecture

1. **Modularité** :
   - Chaque composant peut être utilisé seul ou intégré à d'autres projets.

2. **Interopérabilité** :
   - Les composants communiquent via WebSocket, garantissant une séparation claire des responsabilités.

3. **Scalabilité** :
   - De nouveaux services peuvent être ajoutés facilement en suivant le même modèle.

4. **Flexibilité** :
   - Les utilisateurs peuvent activer uniquement les services nécessaires via `workspace_manager`.

---

## Extensions futures

- Ajout de nouveaux services (ex. gestion des tests, déploiement automatisé).
- Optimisation des performances avec du caching pour les réponses fréquentes.
- Sécurisation des communications WebSocket avec des jetons d'authentification.

---

## Structure des fichiers

```
multi_ai/
├── ai_assistant/
│   ├── src/
│   │   ├── websocket.rs       # Serveur WebSocket pour l'IA
│   │   ├── agent.rs           # Prise de décision IA
│   │   └── ...
├── version_watcher/
│   ├── src/
│   │   ├── websocket.rs       # Serveur WebSocket pour le versioning
│   │   ├── watcher.rs         # Surveillance des fichiers
│   │   └── ...
├── semver_planner/
│   ├── src/
│   │   ├── version.rs         # Gestion des versions SemVer
│   │   └── ...
├── ide_web/
│   ├── src/
│   │   ├── websocket.rs       # Gestion des WebSockets pour l'IDE
│   │   ├── bridge/            # Ponts vers les services
│   │   └── ...
├── workspace_manager/
│   ├── src/
│   │   ├── websocket.rs       # Hub central pour les WebSockets
│   │   └── ...
└── ARCHITECTURE_WEBSOCKET.md  # Documentation de l'architecture WebSocket
```

---

## Contribution

1. Forkez le projet.
2. Créez une branche pour vos modifications :
   ```bash
   git checkout -b feature/ajout-service
   ```
3. Faites vos modifications et committez-les :
   ```bash
   git commit -m "Ajout d'un nouveau service WebSocket"
   ```
4. Poussez vos modifications :
   ```bash
   git push origin feature/ajout-service
   ```
5. Ouvrez une Pull Request.

---

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus d'informations.
