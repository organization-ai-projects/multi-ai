//! Module de cache pour l'IA
//! 
//! Fonctionnement des caches :
//! 1. Les caches utilisent persistent/ (et non memory/ directement) car :
//!    - memory/ définit les structures de base (MemoryNode etc.)
//!    - persistent/ étend ces structures avec save/load
//!    - Le cache a besoin de charger/sauvegarder donc utilise persistent/
//!
//! 2. Chaque cache gère :
//!    - Une HashMap pour stocker les données en RAM
//!    - Un flag "dirty" pour savoir si des modifications sont à sauvegarder
//!    - Le chargement paresseux (lazy loading) : charge depuis le disque uniquement si pas en cache
//!    - La persistance via les méthodes save/load de persistent/
//!
//! 3. Flux typique :
//!    a. get() : 
//!       - Vérifie si présent en cache (RAM)
//!       - Si non, charge depuis le disque via persistent/
//!       - Met en cache pour les prochains accès
//!    b. insert() :
//!       - Met à jour le cache
//!       - Marque comme "dirty"
//!    c. flush() :
//!       - Si "dirty", sauvegarde sur disque via persistent/
//!
//! Les caches servent donc d'intermédiaire optimisé entre la RAM et le disque.

pub mod node_cache;
pub mod link_cache;
pub mod episode_cache;
pub mod artifact_cache;
pub mod journal_cache;
pub mod index_cache;
pub mod cache_manager;

pub use node_cache::NodeCache;
pub use link_cache::LinkCache;
pub use episode_cache::EpisodeCache;
pub use artifact_cache::ArtifactCache;
pub use journal_cache::JournalCache;
pub use index_cache::IndexCache;
pub use cache_manager::CacheManager;
