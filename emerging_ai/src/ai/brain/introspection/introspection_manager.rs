use super::{BrainMetrics, CycleSnapshot, ReplayManager};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct IntrospectionManager {
    brain_metrics: BrainMetrics,
    replay_manager: ReplayManager,
    current_cycle: Option<CycleSnapshot>,
}

impl IntrospectionManager {
    pub fn new() -> Self {
        Self {
            brain_metrics: BrainMetrics::new(),
            replay_manager: ReplayManager::new(),
            current_cycle: None,
        }
    }

    pub async fn begin_cycle(&mut self) -> std::io::Result<Uuid> {
        let cycle_id = Uuid::now_v7();
        self.current_cycle = Some(CycleSnapshot::new(cycle_id));
        Ok(cycle_id)
    }

    pub async fn analyze_history(&mut self, days: i64) -> std::io::Result<Vec<String>> {
        let patterns = self.replay_manager
            .analyze_cycles(Utc::now() - chrono::Duration::days(days))
            .await?;

        let insights = patterns.iter()
            .filter(|p| p.success_rate > 0.8)
            .map(|p| format!("Pattern discovered: {} with success rate {:.2}", 
                p.pattern_id, p.success_rate))
            .collect();

        Ok(insights)
    }

    pub fn record_mutation(&mut self, success: bool, score: f32, code: &str) {
        self.brain_metrics.record_mutation(success, score);
        
        if let Some(cycle) = &mut self.current_cycle {
            cycle.add_mutation(format!(
                "Mutation {} with score {:.2}", 
                if success { "succeeded" } else { "failed" },
                score
            ));
        }
    }

    pub fn suggest_next_goal(&self) -> &'static str {
        if self.brain_metrics.learning_stats.average_score < 0.5 {
            "MaximizeScore"
        } else if self.brain_metrics.learning_stats.patterns_discovered < 5 {
            "ExplorePatterns"
        } else {
            "OptimizeCode"
        }
    }

    pub async fn complete_cycle(&mut self) -> std::io::Result<()> {
        if let Some(cycle) = &self.current_cycle {
            cycle.save().await?;
            
            // Analyse des mutations du cycle
            let mutations_impact = cycle.analyze_mutations();
            for (_, impact) in mutations_impact {
                if impact > 0.5 {
                    self.brain_metrics.learning_stats.patterns_discovered += 1;
                }
            }
        }

        self.current_cycle = None;
        Ok(())
    }
}
