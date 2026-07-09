use crate::monitoring::{HealthMonitor, MetricsCollector, AlertSystem};
use serde_json::json;
use std::time::Instant;
use tokio;

pub struct HumanOrchestrator {
    health_monitor: HealthMonitor,
    metrics_collector: MetricsCollector,
    alert_system: AlertSystem,
}

impl HumanOrchestrator {
    pub fn new() -> Self {
        Self {
            health_monitor: HealthMonitor::new(),
            metrics_collector: MetricsCollector::new(),
            alert_system: AlertSystem::new(),
        }
    }

    pub async fn monitor_cycle(&mut self) -> std::io::Result<()> {
        // 1. Vérification santé système
        let health_status = self.health_monitor.check_system().await?;
        
        // 2. Mise à jour des métriques pour l'interface humaine
        let status = json!({
            "system_status": {
                "healthy": health_status.is_healthy,
                "ram_usage": health_status.ram_usage,
                "cpu_usage": health_status.cpu_usage,
                "disk_space": health_status.disk_space,
            },
            "metrics": self.metrics_collector.get_metrics(),
            "timestamp": chrono::Utc::now(),
        });

        // 3. Sauvegarde du statut pour l'interface humaine
        std::fs::write(
            "human_interface/status.json", 
            serde_json::to_string_pretty(&status)?
        )?;

        // 4. Alertes si nécessaire
        if !health_status.is_healthy {
            self.alert_system.send_alert(
                format!("Système en difficulté: {}", health_status.status_message)
            ).await?;
        }

        Ok(())
    }
}
