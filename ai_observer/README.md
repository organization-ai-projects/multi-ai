# Architecture de l'AI Observer

## Structure des modules

L'architecture suit une organisation en couches avec une séparation claire des responsabilités:

1. **entities/** - Entités et orchestration du système
   - Classes fondamentales et logique du domaine
   - Implémentations concrètes des structures de données et algorithmes
   - Organisation en sous-domaines isolés avec orchestrateurs dédiés
   - Une API d'orchestration centrale (`ai_orchestrator.rs`)

2. **adapters/** - Couche d'adaptation pour les interfaces
   - Adaptateurs, transformations et types d'interface 
   - Point de contact entre les interfaces et les entités du domaine
   - Isole les couches supérieures des détails d'implémentation du moteur IA

3. **systems/** - Orchestrateurs transverses
   - Combinent plusieurs domaines pour des opérations complexes
   - Implémentent la logique qui traverse plusieurs sous-domaines
   - S'appuient sur l'orchestrateur central des entités

4. **system_api.rs** - Façade unifiée du système
   - Point d'entrée unique exposant toutes les capacités du système
   - Interface stable pour les clients externes
   - Ne contient pas de logique métier directe

5. **interfaces/** - Clients de l'API
   - Interfaces utilisateur (CLI, Web, etc.)
   - Dépendent uniquement de system_api.rs

## Principes de découplage

- **Séparation par domaine**: Chaque domaine fonctionnel a ses propres modules et orchestrateurs
- **Dépendances à sens unique**: Chaque couche dépend uniquement des couches inférieures
- **Single point of entry**: `system_api.rs` est le seul point d'entrée pour les interfaces externes
- **Isolation des détails d'implémentation**: Les adaptateurs isolent les interfaces des changements internes
- **Orchestration sans couplage**: Les orchestrateurs utilisent plusieurs sous-domaines sans exposer leurs détails
- **Scalabilité verticale**: Chaque domaine peut évoluer indépendamment

## Avantages pour les tests et la maintenance

- **Testabilité modulaire**: Chaque orchestrateur peut être testé indépendamment
- **Réutilisabilité**: Chaque sous-domaine est conçu pour être réutilisable
- **Évolution découplée**: Un changement dans une entité n'affecte que son sous-domaine
- **Remplacement facilité**: Les implémentations peuvent être changées sans affecter les couches supérieures

## Flux de données
```
┌────────────────────────────────────┐
│ interfaces/ (CLI, UI, etc.)        │
└────────────────┬───────────────────┘
                 │ utilise uniquement
┌────────────────▼───────────────────┐
│ system_api.rs (Façade unifiée)     │
└────────────────┬───────────────────┘
                 │ coordonne
┌────────────────▼───────────────────┐
│ systems/ (Orchestrateurs)          │
└────────────────┬───────────────────┘
                 │ utilisent
┌────────────────▼───────────────────┐
│ adapters/ (Adaptateurs)            │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│ entities/ (Domaine et orchestration)│
└────────────────────────────────────┘
```

## Exemple de flux de traitement
1. L'interface (CLI ou UI) appelle une méthode sur `SystemAPI`
2. `SystemAPI` délègue à l'orchestrateur approprié dans `systems/`
3. L'orchestrateur coordonne en s'appuyant sur l'orchestrateur central des entités
4. L'orchestrateur central des entités coordonne les sous-domaines
5. Les orchestrateurs de sous-domaines appliquent leur logique métier via leurs managers
6. Le résultat remonte la chaîne vers l'interface utilisateur