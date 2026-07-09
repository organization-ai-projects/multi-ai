# 🧠 IA Modulaire en Rust – Architecture `Brain`

Ce projet implémente une intelligence artificielle modulaire inspirée de la cognition humaine. Le cœur du système est un module central `Brain` qui encapsule une mémoire graphique persistante (`PersistentMemoryGraph`) et expose des interfaces spécialisées pour chaque capacité cognitive : perception, cognition, apprentissage, motivation, décision, etc.

---

## 📦 Structure générale du projet

```text
ai/
├── brain/
│   ├── mod.rs                  # Exports des composants
│   ├── brain.rs                # Cerveau central et mémoire
│   └── facades/                # Interfaces spécialisées
│       ├── perception.rs
│       ├── cognition.rs 
│       ├── learning.rs
│       ├── motivation.rs
│       └── dialogue.rs
├── memory/
│   └── persist_memory_graph.rs # Graphe mémoire persistant
├── perception/
│   └── parser.rs               # Analyse et perception textuelle
├── cognition/
│   └── planner.rs              # Raisonnement logique ou symbolique
├── learning/
│   └── learner.rs              # Mécanismes d'apprentissage
├── motivation/
│   └── evaluate.rs             # Gestion des objectifs
├── decision/
│   └── action.rs               # Phase de décision
├── orchestrator/
│   └── brain_orchestrator.rs   # Déroulement complet d’un cycle d’IA
```

---

## 🧠 Le module `Brain`

Le fichier `brain.rs` centralise toute l’intelligence :

* Il contient un champ `memory: PersistentMemoryGraph` qui sert de base à toutes les opérations.
* Il expose des **façades** spécialisées (comme des "interfaces" mentales) pour chaque capacité cognitive :

  * `BrainPerception`
  * `BrainCognition`
  * `BrainLearning`
  * `BrainMotivation` (et d'autres à venir)

Chaque façade donne accès à une portion des fonctionnalités de `Brain`, tout en maintenant l'encapsulation mémoire.

---

## 🧩 Exemple : Ajouter une capacité "Motivation"

### 1. Créer `ai/motivation/mod.rs`

```rust
use crate::ai::brain::BrainMotivation;

pub fn evaluate_goals(motivation: &mut BrainMotivation) {
    motivation.set_goal_priority(42, 0.9);
}
```

---

### 2. Ajouter une façade `BrainMotivation` dans `brain.rs`

```rust
pub fn motivation(&mut self) -> BrainMotivation {
    BrainMotivation { brain: self }
}

pub struct BrainMotivation<'a> {
    pub(crate) brain: &'a mut Brain,
}

impl<'a> BrainMotivation<'a> {
    pub fn set_goal_priority(&mut self, goal_id: usize, priority: f32) {
        self.brain.add_link(goal_id, goal_id, priority);
    }
}
```

---

### 3. Appeler la capacité dans le cycle via l’orchestrateur

Crée ou modifie `ai/orchestrator/brain_orchestrator.rs` :

```rust
use crate::ai::brain::Brain;
use crate::ai::perception::parser::parse_input;
use crate::ai::cognition::planner::reason;
use crate::ai::learning::learner::train;
use crate::ai::motivation::evaluate_goals;

pub fn run_brain_cycle(brain: &mut Brain) {
    {
        let mut perception = brain.perception();
        parse_input("Je suis une IA.", &mut perception);
    }

    {
        let mut cognition = brain.cognition();
        reason(&mut cognition);
    }

    {
        let mut learning = brain.learning();
        train(&mut learning);
    }

    {
        let mut motivation = brain.motivation();
        evaluate_goals(&mut motivation);
    }
}
```

---

### 4. `main.rs` devient ultra simple

```rust
use ai::brain::Brain;
use ai::orchestrator::brain_orchestrator::run_brain_cycle;

fn main() {
    let mut brain = Brain::new();
    let _ = brain.load();

    run_brain_cycle(&mut brain);

    let _ = brain.save();
}
```

---

## ✅ Avantages

| Élément        | Rôle                                       |
| -------------- | ------------------------------------------ |
| `Brain`        | Mémoire + interfaces modulaires            |
| `Perception`   | Analyse d’input (ex: texte)                |
| `Cognition`    | Raisonnement, association                  |
| `Learning`     | Apprentissage et renforcement              |
| `Motivation`   | Définition des objectifs                   |
| `Decision`     | Choix d'une action basée sur les phases    |
| `Orchestrator` | Ordonne les phases du cycle de vie de l’IA |

---

## ➕ Ajouter une nouvelle capacité ( 🧰 perception visuelle, 🗣️ dialogue, 📚 connaissance...)

1. Créez le dossier `ai/<nom_de_la_capacité>/mod.rs`.
2. Créez une façade `Brain<Nom>` dans `brain.rs`.
3. Définissez une fonction comme `process_xxx()` dans le module.
4. Ajoutez-la dans `brain_orchestrator.rs`.

**Exemple récent : Phase de décision**
- Dossier : `ai/decision/action.rs`
- Fonction : `decide_action()`
- Ajoutée dans `brain_orchestrator.rs` pour exécuter une action après les autres phases.

---

## 🧪 À venir

* 📊 Système de logs internes par capacité
* 🧜️ Simulation évolutive avec priorités dynamiques
* 🌐 API pour exposer les capacités via une interface JSON
* 🧱 Registry dynamique des capacités à exécuter par phase
