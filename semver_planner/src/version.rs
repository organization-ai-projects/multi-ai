use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    pub fn bump(&mut self, impact: &crate::planner::Impact) {
        match impact {
            crate::planner::Impact::Major => {
                self.major += 1;
                self.minor = 0;
                self.patch = 0;
            }
            crate::planner::Impact::Minor => {
                self.minor += 1;
                self.patch = 0;
            }
            crate::planner::Impact::Patch => {
                self.patch += 1;
            }
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
