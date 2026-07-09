use crate::planner::Impact;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct Snapshot {
    pub impact: Impact,
    pub hash: String, // Ajout du champ `hash`
}

pub fn get_latest_impact() -> Option<Impact> {
    let path = Path::new(".graphver/snapshots");
    if !path.exists() {
        return None;
    }

    let mut scores = HashMap::new();
    scores.insert("Patch", 1);
    scores.insert("Minor", 2);
    scores.insert("Major", 3);

    let mut max_impact = Impact::Patch;
    let mut max_score = 0;

    for entry in fs::read_dir(path).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension()?.to_str()? != "ron" {
            continue;
        }

        let content = fs::read_to_string(&path).ok()?;
        if let Ok(snapshot) = ron::from_str::<Snapshot>(&content) {
            let s = match snapshot.impact {
                Impact::Patch => 1,
                Impact::Minor => 2,
                Impact::Major => 3,
            };
            if s > max_score {
                max_score = s;
                max_impact = snapshot.impact;
            }
        }
    }

    Some(max_impact)
}
