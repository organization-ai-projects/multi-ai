# 🧠 RHL Roadmap de Développement

Voici les étapes organisées pour faire évoluer ton langage RHL de DSL expérimental → outil de dev complet avec extension VSCode, transpilation, linter et intégration pro.

---

## ✅ Étape 1 : Extension VSCode `vscode-rhl/` (FAIT)

- [x] Coloration syntaxique (`rhl.tmLanguage.json`)
- [x] Configuration langage (`language-configuration.json`)
- [x] Publication sur le Marketplace (publisher `organisationAI`)
- [x] Package et installation `.vsix`

---

## 🔜 Étape 2 : Transpileur / Linter `middle_level/`

- [ ] Transpile `.rhl` → `.rs` (déjà partiellement codé ✅)
- [ ] DSL avec typage (types_mapping.ron)
- [ ] Fonctions dynamiques depuis `.ron`
- [ ] Linter intégré (déjà en place ✅)
- [ ] Génération de fichiers `generated/*.rs`
- [ ] Ajout d’un CLI `middle_level` :
  ```bash
  middle_level transpile dsl_projects/
  middle_level lint dsl_projects/
  ```

---

## 🚀 Étape 3 : Intégration dans l’extension VSCode (RHL Full)

- [ ] Ajouter un serveur LSP ou un `languageClient` JS
- [ ] Exécuter automatiquement la transpilation à la sauvegarde
- [ ] Lancer le linter à l’édition (`onType`) ou `Ctrl+S`
- [ ] Feedback live dans la barre de statut ou sous les lignes
- [ ] Bouton "Run / Transpile RHL" depuis la palette de commandes

---

## 📦 Étape 4 : Packaging complet

- [ ] Publication de `middle_level` sur GitHub (avec binaire Linux/Win)
- [ ] Ajout dans l’extension d’un binaire intégré, ou prérequis système
- [ ] Documentation Markdown `README.md`, `examples/`, `CHANGELOG.md`
- [ ] Ajout d’un `rusth` ou `rhlc` (comme `tsc` pour TypeScript)

---

## 💡 Idée future : Watcher local sans Git

> Tu as demandé :
> > "Est-ce que je peux watcher tout changement de fichier, sans Git, et deviner ce qui a été modifié pour bump automatiquement la version ?"

Oui c’est faisable. Voici comment 👇

### 🛠 Plan technique pour un watcher versionner :

- [ ] Utiliser [`notify`](https://crates.io/crates/notify) en Rust pour écouter les fichiers de `vscode-rhl/`
- [ ] Gérer une config `.bump_rules.ron` :
  ```ron
  (
    rules: [
      (path: "syntaxes/", impact: "patch"),
      (path: "linter/", impact: "patch"),
      (path: "types/", impact: "minor"),
      (path: "dsl_projects/", impact: "major")
    ]
  )
  ```
- [ ] À chaque détection de fichier modifié :
  - Ajouter son impact à une queue
  - Déterminer le plus fort (ex: `major > minor > patch`)
  - Exécuter `vsce publish <version>` automatiquement

Et tu peux faire tourner ce watcher dans `cargo run` du projet `middle_level` pendant que tu codes.

---

Tu veux que je te génère ce watcher Rust prêt à l’emploi ou avec une config RON ?
