//! # Événements de plugin
//!
//! Ce fichier est responsable de:
//! - Définir les types d'événements que les plugins peuvent recevoir
//! - Standardiser la communication avec les plugins
//!
//! Ce fichier NE DOIT PAS contenir:
//! - De la logique de traitement d'événements
//! - Des implémentations concrètes de plugins
//!
//! Il définit uniquement les types d'événements du système de plugins.

/// Événements que les plugins peuvent recevoir
pub enum PluginEvent {
    /// Le plugin a été activé
    Activated,

    /// Le plugin a été désactivé
    Deactivated,

    /// Un message a été reçu via WebSocket
    WebSocketMessage(String),

    /// Navigation vers une nouvelle URI
    Navigate(String),

    /// Demande d'exécution d'une commande
    Command(String, Vec<String>),
}
