mod file_monitor;
mod permission_manager;
mod shared_content;
mod alerts;
mod security_manager;

// Re-exports pour faciliter l'accès
pub use security_manager::SecurityManager;
pub use file_monitor::FileMonitor;
pub use permission_manager::PermissionManager;
pub use shared_content::SharedContentMonitor;
pub use alerts::AlertManager;
