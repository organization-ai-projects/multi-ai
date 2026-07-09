use crate::error::ResourceMonitorError;

#[derive(Clone, PartialEq)]
pub enum AiState {
    Running,
    Paused,
}

#[derive(Clone)]
pub struct AiInstance {
    pub id: String,
    pub state: AiState,
    pub priority: u8,  // 0-255, priorité plus élevée = moins susceptible d'être mis en pause
}

impl AiInstance {
    pub fn new(id: String, priority: u8) -> Self {
        Self {
            id,
            state: AiState::Running,
            priority,
        }
    }
    
    pub fn is_running(&self) -> bool {
        matches!(self.state, AiState::Running)
    }
    
    pub fn is_paused(&self) -> bool {
        matches!(self.state, AiState::Paused)
    }
    
    pub fn update_state_to_paused(&mut self) -> Result<(), ResourceMonitorError> {
        if self.is_running() {
            self.state = AiState::Paused;
            Ok(())
        } else {
            // Déjà en pause
            Ok(())
        }
    }
    
    pub fn update_state_to_running(&mut self) -> Result<(), ResourceMonitorError> {
        if self.is_paused() {
            self.state = AiState::Running;
            Ok(())
        } else {
            // Déjà en marche
            Ok(())
        }
    }
}
