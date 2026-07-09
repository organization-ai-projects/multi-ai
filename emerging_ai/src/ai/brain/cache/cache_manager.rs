use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::interval;
use crate::brain::cache::{NodeCache, LinkCache, EpisodeCache, ArtifactCache, JournalCache, IndexCache};
use uuid::Uuid;
use serde::Serialize;

// Statistiques par type de cache
pub struct CacheStats {
    pub hits: AtomicU64,
    pub misses: AtomicU64,
    pub last_flush: AtomicU64,
    pub total_operations: AtomicU64,
}

impl CacheStats {
    fn new() -> Self {
        Self {
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            last_flush: AtomicU64::new(0),
            total_operations: AtomicU64::new(0),
        }
    }

    fn record_hit(&self) {
        self.hits.fetch_add(1, Ordering::SeqCst);
        self.total_operations.fetch_add(1, Ordering::SeqCst);
    }

    fn record_miss(&self) {
        self.misses.fetch_add(1, Ordering::SeqCst);
        self.total_operations.fetch_add(1, Ordering::SeqCst);
    }
}

pub struct CacheManager {
    node_cache: NodeCache,
    link_cache: LinkCache,
    episode_cache: EpisodeCache,
    artifact_cache: ArtifactCache,
    journal_cache: JournalCache,
    index_cache: IndexCache,
    read_only: AtomicBool,
    stats: CacheStats,
    background_flush_interval: Duration,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            node_cache: NodeCache::new(),
            link_cache: LinkCache::new(),
            episode_cache: EpisodeCache::new(),
            artifact_cache: ArtifactCache::new(),
            journal_cache: JournalCache::new(),
            index_cache: IndexCache::new(),
            read_only: AtomicBool::new(false),
            stats: CacheStats::new(),
            background_flush_interval: Duration::from_secs(300), // 5 minutes par défaut
        }
    }

    // Accesseurs sécurisés pour les caches
    pub fn node_cache(&mut self) -> &mut NodeCache {
        &mut self.node_cache
    }

    pub fn link_cache(&mut self) -> &mut LinkCache {
        &mut self.link_cache
    }

    pub fn episode_cache(&mut self) -> &mut EpisodeCache {
        &mut self.episode_cache
    }

    pub fn artifact_cache(&mut self) -> &mut ArtifactCache {
        &mut self.artifact_cache
    }

    // Configuration
    pub fn set_read_only(&self, value: bool) {
        self.read_only.store(value, Ordering::SeqCst);
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only.load(Ordering::SeqCst)
    }

    pub fn set_flush_interval(&mut self, interval: Duration) {
        self.background_flush_interval = interval;
    }

    // Gestion du flush
    pub async fn start_background_flush(&self) {
        let mut interval = interval(self.background_flush_interval);
        
        loop {
            interval.tick().await;
            if !self.is_read_only() {
                if let Ok(()) = self.flush_all() {
                    self.stats.last_flush.store(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        Ordering::SeqCst
                    );
                }
            }
        }
    }

    pub fn flush_all(&self) -> std::io::Result<()> {
        if self.is_read_only() {
            return Ok(());
        }

        self.node_cache.flush()?;
        self.link_cache.flush()?;
        self.episode_cache.flush()?;
        self.artifact_cache.flush()?;
        Ok(())
    }

    pub fn clear_all(&mut self) {
        if self.is_read_only() {
            return;
        }

        self.node_cache.clear();
        self.link_cache.clear();
        self.episode_cache.clear();
        self.artifact_cache.clear();
    }

    // Statistiques
    pub fn get_stats(&self) -> (u64, u64, u64, u64) {
        (
            self.stats.hits.load(Ordering::SeqCst),
            self.stats.misses.load(Ordering::SeqCst),
            self.stats.last_flush.load(Ordering::SeqCst),
            self.stats.total_operations.load(Ordering::SeqCst),
        )
    }

    pub fn print_stats(&self) {
        let (hits, misses, last_flush, total) = self.get_stats();
        println!("Cache Statistics:");
        println!("Hits: {}", hits);
        println!("Misses: {}", misses);
        println!("Hit ratio: {:.2}%", (hits as f64 / total as f64) * 100.0);
        println!("Last flush: {} seconds ago", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() - last_flush
        );
    }

    pub fn find_nodes_modified_after(&self, timestamp: u64) -> std::io::Result<Vec<Uuid>> {
        // Implémentation déplacée ici
        Ok(vec![])
    }

    pub fn find_links_by_node(&self, node_uuid: Uuid) -> std::io::Result<Vec<Uuid>> {
        // Implémentation déplacée ici
        Ok(vec![])
    }


    pub fn route<T>(&mut self, entity_type: &str, action: &str, data: T) -> std::io::Result<T> {
        // Vérification via policy
        if !self.policy.is_action_allowed(entity_type, action) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("Action {} not allowed for {}", action, entity_type)
            ));
        }

        // Vérification taille cache
        if self.is_cache_full() && action == "save" {
            self.apply_eviction_strategy()?;
        }

        match (entity_type, action) {
            // Actions standards pour tous les types (y compris journal et index)
            (_, "save") => self.save_data(entity_type, data.uuid, &bincode_next::encode_to_vec(&data, bincode_next::config::standard())?),
            (_, "get") => self.get_data(entity_type, data),
            (_, "list") => self.list_entities(entity_type),
            (_, "save_legacy") => self.save_legacy(entity_type, data),
            (_, "batch_save") => self.batch_save(entity_type, data),
            
            // Default
            _ => Ok(())
        }
    }

    fn is_cache_full(&self) -> bool {
        let total_size = self.node_cache.len() + 
                        self.link_cache.len() + 
                        self.episode_cache.len() + 
                        self.artifact_cache.len();
        total_size >= self.policy.max_cache_size
    }

    fn apply_eviction_strategy(&mut self) -> std::io::Result<()> {
        match self.policy.eviction_strategy {
            EvictionStrategy::LRU => self.evict_lru(),
            EvictionStrategy::FIFO => self.evict_fifo(),
            EvictionStrategy::Custom(ref strategy) => self.evict_custom(strategy),
        }
    }

    // Méthodes privées pour le routage de journal et index
    fn handle_journal<T>(&mut self, entity_type: &str, action: &str, data: T) -> std::io::Result<()> {
        match action {
            "save" => {
                let (action, entity_type, uuid) = data;
                self.journal_cache.add_entry(action, entity_type, uuid);
                Ok(())
            },
            "complete" => {
                self.journal_cache.mark_completed(data);
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn handle_index<T>(&mut self, entity_type: &str, data: T) -> std::io::Result<()> {
        let (uuid, timestamp) = data;
        match entity_type {
            "node" => self.index_cache.add_node(uuid, timestamp),
            "link" => self.index_cache.add_link(uuid, timestamp),
            _ => return Ok(())
        }
        Ok(())
    }

    // Rendu privé car maintenant accessible via route()
    fn batch_save(&self, entity_type: &str, uuids: &[Uuid]) -> std::io::Result<()> {
        match entity_type {
            "node" => self.node_cache.batch_save(uuids),
            "link" => self.link_cache.batch_save(uuids),
            "code" => self.artifact_cache.batch_save_code(uuids),
            "log" => self.artifact_cache.batch_save_logs(uuids),
            "image" => self.artifact_cache.batch_save_images(uuids),
            _ => Ok(())
        }
    }

    // Les autres méthodes deviennent privées car tout passe par route()
    fn save_legacy<T: Serialize>(&mut self, entity_type: &str, data: T) -> std::io::Result<()> {
        let serialized = bincode_next::encode_to_vec(&data, bincode_next::config::standard())?;
        match entity_type {
            "node" => self.save_data("node", uuid, &serialized),
            "link" => self.save_data("link", uuid, &serialized),
            "code" => self.save_data("code", uuid, &serialized),
            "episode" => self.save_data("episode", uuid, &serialized),
            "log" => self.save_data("log", uuid, &serialized),
            "image" => self.save_data("image", uuid, &serialized),
            "journal" => self.save_data("journal", uuid, &serialized),
            "index" => self.save_data("index", uuid, &serialized),
            _ => Ok(())
        }
    }

    fn save_data(&mut self, entity_type: &str, uuid: Uuid, data: &[u8]) -> std::io::Result<()> {
        match entity_type {
            "node" => self.node_cache.insert(data),
            "link" => self.link_cache.insert(data), 
            "code" => self.artifact_cache.insert_code(data),
            "episode" => self.episode_cache.insert(data),
            "log" => self.artifact_cache.insert_log(data),
            "image" => self.artifact_cache.insert_image(data),
            "journal" => self.journal_cache.insert(data),
            "index" => self.index_cache.insert(data),
            _ => Ok(())
        }
    }

    fn get_data(&mut self, entity_type: &str, uuid: Uuid) -> std::io::Result<Option<&dyn std::fmt::Debug>> {
        match entity_type {
            "node" => Ok(self.node_cache.get(&uuid)),
            "link" => Ok(self.link_cache.get(&uuid)),
            "code" => Ok(self.artifact_cache.get_code(&uuid)),
            "episode" => Ok(self.episode_cache.get(&uuid)),
            "log" => Ok(self.artifact_cache.get_log(&uuid)),
            "image" => Ok(self.artifact_cache.get_image(&uuid)), 
            "journal" => Ok(self.journal_cache.get(&uuid)),
            "index" => Ok(self.index_cache.get(&uuid)),
            _ => Ok(None)
        }
    }

    fn list_entities(&self, entity_type: &str) -> std::io::Result<Vec<Uuid>> {
        match entity_type {
            "node" => Ok(self.node_cache.list()),
            "link" => Ok(self.link_cache.list()),
            "code" => Ok(self.artifact_cache.list_codes()),
            "episode" => Ok(self.episode_cache.list()),
            "log" => Ok(self.artifact_cache.list_logs()),
            "image" => Ok(self.artifact_cache.list_images()),
            "journal" => Ok(self.journal_cache.list()),
            "index" => Ok(self.index_cache.list()),
            _ => Ok(vec![])
        }
    }

    // Support des transactions
    fn begin_transaction(&mut self) -> std::io::Result<()> {
        self.journal_cache.add_entry("BEGIN", "transaction", &Uuid::new_v4().to_string());
        Ok(())
    }

    fn commit_transaction(&mut self) -> std::io::Result<()> {
        self.flush_all()?;
        self.journal_cache.add_entry("COMMIT", "transaction", &Uuid::new_v4().to_string());
        Ok(())
    }

    fn rollback_transaction(&mut self) -> std::io::Result<()> {
        self.journal_cache.restore_incomplete_operations();
        Ok(())
    }

    // Gestion du flush atomique
    fn atomic_flush(&mut self) -> std::io::Result<()> {
        self.begin_transaction()?;
        if let Err(e) = self.flush_all() {
            self.rollback_transaction()?;
            return Err(e);
        }
        self.commit_transaction()
    }

    fn evict_lru(&mut self) -> std::io::Result<()> {
        // On évince le moins récemment utilisé dans chaque cache si nécessaire
        if self.node_cache.len() > 0 {
            let oldest_uuid = self.node_cache.get_lru_uuid()?;
            self.node_cache.remove(&oldest_uuid);
        }
        if self.link_cache.len() > 0 {
            let oldest_uuid = self.link_cache.get_lru_uuid()?;
            self.link_cache.remove(&oldest_uuid);
        }
        if self.episode_cache.len() > 0 {
            let oldest_uuid = self.episode_cache.get_lru_uuid()?;
            self.episode_cache.remove(&oldest_uuid);
        }
        if self.artifact_cache.code_cache_len() > 0 {
            let oldest_uuid = self.artifact_cache.get_lru_code_uuid()?;
            self.artifact_cache.remove_code(&oldest_uuid);
        }
        Ok(())
    }

    fn evict_fifo(&mut self) -> std::io::Result<()> {
        // On évince le premier entré dans chaque cache si nécessaire
        if self.node_cache.len() > 0 {
            let first_uuid = self.node_cache.get_first_uuid()?;
            self.node_cache.remove(&first_uuid);
        }
        if self.link_cache.len() > 0 {
            let first_uuid = self.link_cache.get_first_uuid()?;
            self.link_cache.remove(&first_uuid);
        }
        if self.episode_cache.len() > 0 {
            let first_uuid = self.episode_cache.get_first_uuid()?;
            self.episode_cache.remove(&first_uuid);
        }
        if self.artifact_cache.code_cache_len() > 0 {
            let first_uuid = self.artifact_cache.get_first_code_uuid()?;
            self.artifact_cache.remove_code(&first_uuid);
        }
        Ok(())
    }

    fn evict_custom(&mut self, strategy: &str) -> std::io::Result<()> {
        match strategy {
            // Éviction basée sur le timestamp
            "oldest" => {
                let oldest_timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                // Pour chaque cache, on trouve et supprime l'entité la plus ancienne
                if let Some(oldest) = self.find_nodes_modified_after(oldest_timestamp)?.first() {
                    self.node_cache.remove(oldest);
                }
                if let Some(oldest) = self.find_links_by_node(*oldest)?.first() {
                    self.link_cache.remove(oldest);
                }
            },

            // Éviction basée sur la taille
            "largest" => {
                let largest_node = self.node_cache.find_largest()?;
                if let Some(uuid) = largest_node {
                    self.node_cache.remove(&uuid);
                }
                let largest_link = self.link_cache.find_largest()?;
                if let Some(uuid) = largest_link {
                    self.link_cache.remove(&uuid);
                }
            },

            // Éviction aléatoire
            "random" => {
                use rand::Rng;
                let mut rng = rand::rng();
                
                if self.node_cache.len() > 0 {
                    let random_uuid = self.node_cache.get_random_uuid(&mut rng)?;
                    self.node_cache.remove(&random_uuid);
                }
                if self.link_cache.len() > 0 {
                    let random_uuid = self.link_cache.get_random_uuid(&mut rng)?;
                    self.link_cache.remove(&random_uuid);
                }
            },

            _ => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown eviction strategy: {}", strategy)
            ))
        }
        Ok(())
    }
}
