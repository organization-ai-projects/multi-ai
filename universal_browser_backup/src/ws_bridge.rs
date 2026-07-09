//! # Bridge WebSocket
//!
//! Ce fichier est responsable de:
//! - Gérer les connexions WebSocket
//! - Permettre la communication avec les services externes
//! - Distribuer les messages reçus aux composants appropriés
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques d'interface utilisateur
//! - Des définitions de composants visuels
//! - Des logiques métier spécifiques aux plugins
//!
//! Il sert uniquement de pont de communication.

use crate::plugins::PluginEvent;
use std::collections::VecDeque;

/// État de la connexion WebSocket
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    /// Non connecté
    Disconnected,

    /// En cours de connexion
    Connecting,

    /// Connecté
    Connected,

    /// En cours de déconnexion
    Disconnecting,
}

/// Message WebSocket
#[derive(Debug, Clone)]
pub struct WsMessage {
    /// Le type de message
    pub message_type: String,

    /// Le contenu du message
    pub content: String,
}

/// Bridge WebSocket
pub struct WsBridge {
    /// L'URL de connexion
    url: String,

    /// L'état de la connexion
    state: ConnectionState,

    /// Les messages en attente d'envoi
    outgoing_messages: VecDeque<WsMessage>,

    /// Les messages reçus
    incoming_messages: VecDeque<WsMessage>,
}

impl WsBridge {
    /// Crée un nouveau bridge WebSocket
    pub fn new() -> Self {
        Self {
            url: "ws://localhost:8080".to_string(),
            state: ConnectionState::Disconnected,
            outgoing_messages: VecDeque::new(),
            incoming_messages: VecDeque::new(),
        }
    }

    /// Configure l'URL de connexion
    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    /// Se connecte au serveur WebSocket
    pub fn connect(&mut self) -> Result<(), String> {
        // Dans une implémentation réelle, on se connecterait vraiment au serveur
        // Pour l'instant, on simule la connexion

        self.state = ConnectionState::Connecting;

        // Simuler une connexion réussie
        self.state = ConnectionState::Connected;

        Ok(())
    }

    /// Se déconnecte du serveur WebSocket
    pub fn disconnect(&mut self) {
        // Dans une implémentation réelle, on se déconnecterait proprement
        // Pour l'instant, on simule la déconnexion

        self.state = ConnectionState::Disconnecting;

        // Simuler une déconnexion réussie
        self.state = ConnectionState::Disconnected;
    }

    /// Envoie un message au serveur WebSocket
    pub fn send_message(&mut self, message_type: &str, content: &str) -> Result<(), String> {
        if self.state != ConnectionState::Connected {
            return Err("Non connecté au serveur WebSocket".to_string());
        }

        let message = WsMessage {
            message_type: message_type.to_string(),
            content: content.to_string(),
        };

        self.outgoing_messages.push_back(message);

        Ok(())
    }

    /// Met à jour le bridge WebSocket (à appeler régulièrement)
    pub fn update(&mut self) -> Vec<PluginEvent> {
        let mut events = Vec::new();

        // Traiter les messages sortants
        while let Some(message) = self.outgoing_messages.pop_front() {
            // Dans une implémentation réelle, on enverrait le message au serveur
            // Pour l'instant, on simule l'envoi

            // Simuler une réponse pour les tests
            if message.message_type == "ia_query" {
                self.incoming_messages.push_back(WsMessage {
                    message_type: "ia_suggestion".to_string(),
                    content: "Voici une suggestion basée sur votre requête".to_string(),
                });
            }
        }

        // Traiter les messages entrants
        while let Some(message) = self.incoming_messages.pop_front() {
            // Créer un événement à partir du message
            let event = PluginEvent::WebSocketMessage(format!(
                "{}:{}",
                message.message_type, message.content
            ));

            events.push(event);
        }

        events
    }

    /// Vérifie si le bridge est connecté
    pub fn is_connected(&self) -> bool {
        self.state == ConnectionState::Connected
    }

    /// Retourne l'état actuel de la connexion
    pub fn connection_state(&self) -> ConnectionState {
        self.state
    }
}
