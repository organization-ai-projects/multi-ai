# 📄 DESIGN.md — Option B : Appel des façades dans chaque logique métier

## 🔐 Principe de l'architecture (Option B)

Dans cette architecture, **chaque fichier de logique métier (learning.rs, perception.rs, etc.) est responsable d'appeler la façade correspondante**. Les façades utilisent un **singleton global `Brain`** pour accéder à la mémoire, éliminant ainsi le besoin de passer une instance explicite de `Brain`.

➡️ `brain_orchestrator.rs` est le **seul autorisé à orchestrer les appels aux logiques métier**, mais il **ne doit jamais appeler directement `Brain` ni manipuler ses façades**. Il doit uniquement utiliser les fonctions exposées par les modules métiers.

Ce modèle vise à :

* Garantir la **cohérence locale** des modules (chaque fichier gère sa propre activité)
* Éviter le couplage croissant autour d'un point unique d'orchestration globale
* Favoriser la testabilité, la lisibilité et la logique modulaire

---

## 🛋️ Structure et flux

```text
brain.rs → expose un singleton global (Brain::get_mut())

learning/learner.rs
   └→ appelle directement BrainLearning

dialogue/dialogue_engine.rs
   └→ appelle directement BrainDialogue

brain_orchestrator.rs
   └→ appelle les fonctions orchestrées uniquement (ex: run_learning())
```

---

## 📝 Exemple complet

### brain.rs

```rust
use once_cell::sync::Lazy;
use std::cell::RefCell;

pub struct Brain {
    // ...existing code...
}

impl Brain {
    // Singleton Brain global
    static GLOBAL_BRAIN: Lazy<RefCell<Brain>> = Lazy::new(|| RefCell::new(Brain::new()));

    pub fn get_mut() -> std::cell::RefMut<'static, Brain> {
        GLOBAL_BRAIN.borrow_mut()
    }
}
```

### learning/learner.rs

```rust
use crate::ai::brain::BrainLearning;

pub fn run_learning() {
    let mut learning = BrainLearning::new();
    learning.reinforce_success(42);
    // logique supplémentaire...
}
```

### dialogue/dialogue_engine.rs

```rust
use crate::ai::brain::BrainDialogue;

pub fn handle_dialogue(text: &str) {
    let mut dialogue = BrainDialogue::new();
    dialogue.store_dialogue(text);
    // logique supplémentaire...
}
```

### brain_orchestrator.rs

```rust
use crate::learning::learner::run_learning;
use crate::dialogue::dialogue_engine::handle_dialogue;

pub fn orchestrate_cycle() {
    run_learning();
    handle_dialogue("Bonjour !");
}
```

---

## ❌ Ce qu'on NE fait PAS dans l'option B

* ❌ Pas de passage explicite de `Brain` dans les modules métiers
* ❌ Pas d'appel direct à `Brain::get_mut()` en dehors des façades
* ❌ Pas de gestion centralisée des façades dans un fichier unique
* ❌ Pas d'import massif de toutes les logiques métiers dans un seul fichier

✅ Chaque module utilise sa propre façade pour accéder à `Brain` via le singleton global.

---

## 🎉 Avantages de l'option B

| Avantage                  | Description                                                 |
| ------------------------- | ----------------------------------------------------------- |
| ✅ Localisation forte      | Chaque module est autonome et lisible                       |
| ✅ Encapsulation claire    | Le `Brain` gère la mémoire, les modules gèrent leur logique |
| ✅ Zéro couplage croisant  | Aucun point de convergence rigide                           |
| ✅ Extensibilité naturelle | Ajouter une capacité = 1 module + 1 façade + 1 appel local  |

---

## 💼 Conclusion

**Option B est la structure retenue.**

Elle impose une responsabilité claire, une architecture modulaire, et évite toute centralisation contre-productive. Chaque capacité est responsable de sa propre orchestration, et `Brain` reste le garant unique de l'accès mémoire via un singleton global.
