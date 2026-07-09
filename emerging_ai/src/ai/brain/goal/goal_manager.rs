use super::intent::{GoalType, Intent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct GoalManager {
    active_goals: HashMap<Uuid, Goal>,
    completed_goals: Vec<Goal>,
    current_focus: Option<Uuid>,
}

impl GoalManager {
    pub fn new() -> Self {
        Self {
            active_goals: HashMap::new(),
            completed_goals: Vec::new(),
            current_focus: None,
        }
    }

    pub fn add_goal(&mut self, goal_type: &str, priority: u8) -> Uuid {
        let goal = Goal {
            uuid: Uuid::now_v7(),
            goal_type: goal_type.to_string(),
            priority,
            progress: 0.0,
            created_at: chrono::Utc::now(),
            completed_at: None,
        };
        let uuid = goal.uuid;
        self.active_goals.insert(uuid, goal);
        uuid
    }

    pub fn update_progress(&mut self, uuid: Uuid, progress: f32) -> bool {
        if let Some(goal) = self.active_goals.get_mut(&uuid) {
            goal.progress = progress;
            if progress >= 1.0 {
                if let Some(completed) = self.active_goals.remove(&uuid) {
                    let mut completed = completed;
                    completed.completed_at = Some(chrono::Utc::now());
                    self.completed_goals.push(completed);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_next_goal(&self) -> Option<Uuid> {
        self.active_goals
            .iter()
            .max_by_key(|(_, goal)| goal.priority)
            .map(|(uuid, _)| *uuid)
    }

    pub fn abandon_goal(&mut self, uuid: Uuid) {
        self.active_goals.remove(&uuid);
    }

    pub fn get_goal_history(&self) -> Vec<&Goal> {
        self.completed_goals.iter().collect()
    }

    pub fn generate_goal(&mut self) -> Uuid {
        let goals_analysis = self.analyze_goals_history();

        let new_goal = match goals_analysis {
            GoalAnalysis::StagnantScore => Goal::new("ExplorePatterns", 90), // Haute priorité pour sortir de la stagnation
            GoalAnalysis::HighComplexity => Goal::new("OptimizeCode", 85), // Priorité pour simplifier
            GoalAnalysis::LowDiversity => Goal::new("LearnNewPattern", 80), // Priorité pour diversifier
            _ => Goal::new("MaximizeScore", 70),                            // But par défaut
        };

        let uuid = new_goal.uuid;
        self.active_goals.insert(uuid, new_goal);
        uuid
    }

    fn analyze_goals_history(&self) -> GoalAnalysis {
        let recent_goals = self
            .completed_goals
            .iter()
            .rev()
            .take(10)
            .collect::<Vec<_>>();

        if recent_goals.iter().all(|g| g.progress < 0.3) {
            GoalAnalysis::StagnantScore
        } else if recent_goals
            .iter()
            .filter(|g| g.goal_type == "OptimizeCode")
            .count()
            < 2
        {
            GoalAnalysis::HighComplexity
        } else if recent_goals
            .iter()
            .map(|g| &g.goal_type)
            .collect::<std::collections::HashSet<_>>()
            .len()
            < 3
        {
            GoalAnalysis::LowDiversity
        } else {
            GoalAnalysis::Normal
        }
    }
}

#[derive(Debug)]
enum GoalAnalysis {
    StagnantScore,
    HighComplexity,
    LowDiversity,
    Normal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Goal {
    uuid: Uuid,
    goal_type: String,
    priority: u8,
    progress: f32,
    created_at: chrono::DateTime<chrono::Utc>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Goal {
    fn new(goal_type: &str, priority: u8) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            goal_type: goal_type.to_string(),
            priority,
            progress: 0.0,
            created_at: chrono::Utc::now(),
            completed_at: None,
        }
    }
}
