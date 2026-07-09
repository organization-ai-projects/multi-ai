# Managers de la Mémoire Individuelle

Ce module contient les gestionnaires de niveau intermédiaire qui orchestrent les opérations sur la mémoire individuelle de l'IA.

## Composants prévus

- `node_manager.rs` - Gestion des nœuds de mémoire individuelle (création, mise à jour, suppression)
- `relation_manager.rs` - Gestion des relations bidirectionnelles entre nœuds avec scores d'efficacité
- `memory_lifecycle_manager.rs` - Gestion de la progression des souvenirs entre mémoire court, moyen et long terme
- `retrieval_manager.rs` - Gestion des opérations de récupération et de recherche dans la mémoire
- `inference_manager.rs` - Gestion des inférences et déductions basées sur la mémoire
- `exposure_weight_manager.rs` - Gestion des poids et scores basés sur la fréquence d'exposition

## Modèle de mémoire

Le système implémente un modèle multi-niveaux avec des caractéristiques spécifiques :

### Niveaux de mémoire
- **Mémoire à court terme** : Informations récentes, volatile mais rapidement accessible
- **Mémoire à moyen terme** : Informations stabilisées, partiellement consolidées
- **Mémoire à long terme** : Informations permanentes, fortement consolidées

### Progression des souvenirs
- Les souvenirs transitent progressivement du court au long terme
- Le `memory_lifecycle_manager.rs` gère cette progression selon des règles établies
- Aucune information n'est réellement "oubliée", mais devient moins accessible

### Système de pondération
- Chaque élément possède un poids basé sur la fréquence d'exposition
- L'exposition répétée renforce le poids et accélère la consolidation
- Le `exposure_weight_manager.rs` maintient ces scores et poids

### Relations bidirectionnelles
- Toutes les relations entre nœuds sont bidirectionnelles
- Chaque relation possède un score d'efficacité positif ou négatif
- Les scores positifs indiquent des associations utiles/correctes
- Les scores négatifs signalent des associations incorrectes/inefficaces
- Ces scores évoluent avec l'expérience et l'apprentissage

### Conservation intégrale des informations
- **Aucun pruning** : Contrairement aux systèmes conventionnels, aucune information n'est jamais supprimée
- Les souvenirs moins utilisés deviennent moins accessibles mais restent toujours récupérables
- La réexposition ou le rappel volontaire peut faire "remonter" des souvenirs anciens
- Cette approche permet de conserver l'intégralité de l'historique expérientiel de l'IA

## Responsabilités

- Exposer des opérations de haut niveau pour manipuler les composants spécifiques de la mémoire individuelle
- Implémenter la logique métier spécifique à la mémoire individuelle
- Coordonner l'utilisation des composants bas niveau
- Assurer la cohérence des données lors des opérations complexes

## Construction du graphe de mémoire

La construction et la gestion du graphe de mémoire complet est une responsabilité de l'orchestrateur, pas des managers individuels :

- Chaque manager gère son domaine spécifique (nœuds, relations, etc.)
- L'orchestrateur du sous-domaine (`orchestrator_individual_memory.rs`) :
  - Combine les fonctionnalités des différents managers pour construire et maintenir le graphe complet
  - Coordonne les interactions entre tous les managers
  - Expose une API complète pour le graphe de mémoire à l'orchestrateur principal
  - Assure la cohérence globale du graphe pendant les opérations complexes

Cette structuration assure que :
- Chaque manager reste focalisé sur sa responsabilité spécifique
- Les managers restent totalement isolés les uns des autres
- L'orchestrateur centralise la logique d'assemblage du graphe complet
- Aucune dépendance circulaire ou ambiguïté de responsabilité n'existe

## Interface

- Chaque manager expose une API claire pour l'orchestrateur UNIQUEMENT
- Les managers n'interagissent JAMAIS directement entre eux
- Toute communication entre managers passe OBLIGATOIREMENT par l'orchestrateur
- Les managers accèdent aux composants de bas niveau via leurs interfaces publiques

## Intégration avec l'orchestrateur

- L'orchestrateur `orchestrator_individual_memory.rs` instancie et coordonne tous les managers
- Les managers sont injectés dans l'orchestrateur via des traits spécifiques
- Les dépendances externes (services, configuration) sont fournies par l'orchestrateur

## Considérations techniques

- La séparation des responsabilités entre managers doit être clairement définie
- Les managers doivent être testables individuellement
- Les opérations coûteuses doivent être optimisées ou asynchrones
- La gestion des erreurs doit être cohérente entre tous les managers

## Isolation du module

Les managers de ce module sont isolés et ne peuvent pas être accédés directement par d'autres sous-domaines. Toute interaction avec ces managers doit passer par l'orchestrateur du sous-domaine.