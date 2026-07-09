consciencious_ai/
│
├── ai/                         # Composants "vivants" uniquement
│   ├── genome/                 # Traits & plans génétiques évolutifs
│   │   ├── mod.rs             # Configuration genome
│   │   ├── evolution/         # Mécanismes d'évolution
│   │   │   ├── mod.rs
│   │   │   ├── mutation.rs    # Mutations des traits
│   │   │   └── selection.rs   # Sélection naturelle
│   │   ├── brain/             # Miroir des traits du cerveau
│   │   │   ├── mod.rs
│   │   │   ├── subconscious/  # Traits du subconscient
│   │   │   │   ├── watcher.rs
│   │   │   │   └── intuition.rs
│   │   │   ├── unconscious/   # Traits de l'inconscient
│   │   │   │   ├── routines.rs
│   │   │   │   └── drives.rs
│   │   │   ├── conscious/     # Traits du conscient
│   │   │   │   ├── attention.rs
│   │   │   │   ├── planner.rs
│   │   │   │   └── goals.rs
│   │   │   └── memory/        # Traits mémoriels
│   │   │       ├── short_term.rs
│   │   │       ├── long_term.rs
│   │   │       └── episodic.rs
│   │   │
│   │   └── body/              # Miroir des traits physiques
│   │       ├── mod.rs
│   │       ├── sensors/       # Traits des capteurs
│   │       │   ├── audio.rs
│   │       │   ├── vision.rs
│   │       │   ├── tactile.rs
│   │       │   └── generic.rs
│   │       └── actuators/     # Traits des actuateurs
│   │           ├── voice.rs
│   │           ├── motion.rs
│   │           ├── display.rs
│   │           └── io.rs
│   │
│   ├── brain/                 # "Esprit", logique cognitive multi-couches
│   │   ├── mod.rs
│   │   ├── subconscious/     # Subconscient : surveillance et intuition
│   │   │   ├── mod.rs
│   │   │   ├── state.rs      # État interne subconscient
│   │   │   ├── strategy.rs   # Stratégies de filtrage
│   │   │   ├── watcher.rs    # Surveillance totale
│   │   │   └── intuition.rs  # Prédiction, triggers
│   │   │
│   │   ├── unconscious/      # Inconscient : automatismes
│   │   │   ├── mod.rs
│   │   │   ├── state.rs      # État interne inconscient
│   │   │   ├── strategy.rs   # Stratégies d'automatisation
│   │   │   ├── routines.rs   # Habitudes, réflexes
│   │   │   └── drives.rs     # Pulsions, besoins
│   │   │
│   │   ├── conscious/        # Conscient : processus actifs
│   │   │   ├── mod.rs
│   │   │   ├── state.rs      # État interne conscient
│   │   │   ├── strategy.rs   # Stratégies décisionnelles
│   │   │   ├── attention.rs  # Focus 
│   │   │   ├── planner.rs    # Planification
│   │   │   └── goals.rs      # Objectifs
│   │   │
│   │   ├── memory/           # Système de mémoire 
│   │   │   ├── mod.rs        # Export de l'API mémoire
│   │   │   ├── base_memory.rs # Types et traits fondamentaux
│   │   │   ├── nodes.rs      # Définitions base noeuds
│   │   │   ├── links.rs      # Définitions base liens
│   │   │   ├── episode.rs    # Définitions base épisodes
│   │   │   │
│   │   │   ├── ram/         # Mémoire vive
│   │   │   │   ├── mod.rs
│   │   │   │   ├── nodes.rs 
│   │   │   │   ├── links.rs
│   │   │   │   ├── episode.rs
│   │   │   │   └── artifacts/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── code.rs
│   │   │   │       ├── image.rs
│   │   │   │       └── log.rs
│   │   │   │
│   │   │   ├── persistent/  # Stockage disque
│   │   │   │   ├── mod.rs
│   │   │   │   ├── nodes.rs
│   │   │   │   ├── links.rs
│   │   │   │   ├── episode.rs
│   │   │   │   └── artifacts/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── code.rs
│   │   │   │       ├── image.rs
│   │   │   │       └── log.rs
│   │   │   │
│   │   │   └── cache/      # Cache rapide
│   │   │       ├── mod.rs
│   │   │       ├── nodes.rs
│   │   │       ├── links.rs
│   │   │       ├── episode.rs
│   │   │       └── artifacts/
│   │   │           ├── mod.rs
│   │   │           ├── code.rs
│   │   │           ├── image.rs
│   │   │           └── log.rs
│   │   │
│   │   ├── bus/             # Bus interne
│   │   │   ├── mod.rs
│   │   │   ├── event.rs     # Définition des événements
│   │   │   ├── router.rs    # Routage inter-strates
│   │   │   └── handlers/    # Handlers par type d'event
│   │   │       ├── memory_access.rs
│   │   │       ├── action_request.rs
│   │   │       └── signal.rs
│   │   └── api.rs           # Interface cerveau
│   │
│   ├── body/                # Interface physique
│       ├── mod.rs
│       ├── sensors/        
│       │   ├── audio.rs    
│       │   ├── vision.rs    
│       │   ├── tactile.rs  
│       │   └── generic.rs  
│       └── actuators/      
│           ├── voice.rs    
│           ├── motion.rs   
│           ├── display.rs  
│           └── io.rs       
│
│   └── multiagent/          # Coordination multi-IA
│       ├── mod.rs
│       ├── manager.rs       # Gestion population
│       └── bus.rs          # Communication inter-agent
│
├── human/                    # Outils externes pour humains/devs
│   ├── monitoring/          # Surveillance et métriques
│   │   ├── mod.rs
│   │   ├── health_monitor.rs  # Surveillance santé système et alertes
│   │   ├── metrics.rs         # Collecte et agrégation des métriques
│   │   └── logs.rs           # Gestion des logs et traces
│   ├── tools/              # Outils d'administration
│   │   ├── mod.rs
│   │   ├── replay.rs         # Rejouer des séquences d'événements passés
│   │   └── dataset_builder.rs # Construction/préparation des datasets
│   └── ui/                 # Interfaces utilisateur
│       ├── mod.rs
│       ├── dashboard.rs      # Interface principale de monitoring
│       └── inspector.rs      # Outil d'inspection détaillée des états
│
├── orchestrator/      # Gestion du cycle de vie
│   ├── mod.rs
│   └── orchestrator.rs # Coordination des composants et cycle de vie
│
├── config/     # Configuration système
│   └── mod.rs  # Chargement et validation des configs
│
├── main.rs                  # Point d'entrée
└── Cargo.toml
