# Architecture d'Auto-Versioning

## 🎯 Vue d'ensemble

Auto-Versioning est un système intelligent de gestion de versions qui :
- Surveille les changements de code en temps réel
- Analyse leur impact sur le versioning sémantique
- Apprend des patterns et décisions passées
- S'auto-améliore avec le temps

## 🧠 Système d'Intelligence

### Composants Clés

1. **BrainLearner** (`src/brain/learner.rs`)
   - Gère l'apprentissage et les prédictions
   - Stocke l'historique des décisions
   - S'adapte au contexte du projet

2. **DecisionTreeLearner** (`src/brain/decision.rs`)
   - Arbre de décision pour l'analyse de patterns
   - Features : comptage de mots-clés, contexte, etc.
   - Amélioration continue des prédictions

3. **PatternAnalyzer** (`src/analyzer.rs`)
   - Détecte les patterns dans le code
   - Évalue l'impact des changements
   - Utilise l'historique pour la précision

### Stockage & Persistence

- `.av-brain/` : Dossier contenant les données d'apprentissage
  - `brain.ron` : Configuration lisible (debug)
  - `brain.bin` : Données binaires (performance)

### Modes d'Apprentissage

1. **Mode Local**
   ```rust
   LearningMode::Local
   // - Apprentissage uniquement sur le projet
   // - Pas de partage de données
   // - Plus rapide mais moins intelligent
   ```

2. **Mode Collaboratif**
   ```rust
   LearningMode::Collaborative
   // - Apprentissage partagé
   // - Patterns communautaires
   // - Intelligence collective
   ```

## 🔄 Workflow Principal

1. **Détection de Changement**
   ```rust
   // src/watcher.rs
   pub struct VersionWatcher {
       // Surveillance en temps réel
       // Filtrage des changements pertinents
       // Analyse immédiate
   }
   ```

2. **Analyse d'Impact**
   ```rust
   // src/analyzer.rs
   pub fn determine_impact() {
       // 1. Extraction des features
       // 2. Prédiction par le cerveau
       // 3. Affinage par l'arbre de décision
       // 4. Décision finale pondérée
   }
   ```

3. **Application & Feedback**
   ```rust
   // src/brain/learner.rs
   impl BrainLearner {
       // Apprentissage du résultat
       // Ajustement des poids
       // Mise à jour des patterns
   }
   ```

## 📊 Visualisation & API

- **API REST** : `/status`, `/bump`, `/graph`
- **Webhooks** : Discord, GitHub
- **Interface Web** : Graphe de dépendances

## 🛠️ Configuration

### project.avconfig.ron
```ron
(
    patterns: [/* Règles de détection */],
    feedback_enabled: true,
    learning_mode: "collaborative",
    confidence_threshold: 0.8
)
```

### Types d'Intelligence
- **Modèle Principal** : Apprentissage par renforcement
- **Arbre de Décision** : Classification des changements
- **Analyse Contextuelle** : Prise en compte de l'environnement
