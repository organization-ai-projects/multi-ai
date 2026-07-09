pub mod sandbox;
pub mod file_guard;
pub mod intrusion_detector;
pub mod cryptography;
pub mod security_zones;
pub mod knowledge_sharing;
pub mod sandbox_manager;

// Re-exports pour faciliter l'accès
pub use sandbox::Sandbox;
pub use sandbox::IsolationLevel;
pub use file_guard::FileGuard;
pub use intrusion_detector::IntrusionDetector;
pub use intrusion_detector::ActivityEvent;
pub use security_zones::{SecurityZoneManager, AccessLevel};
pub use knowledge_sharing::KnowledgeSharing;
pub use sandbox_manager::{SandboxManager, SandboxFeature, SandboxStatus};
