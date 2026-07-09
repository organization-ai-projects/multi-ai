#[derive(Debug, Clone)]
pub struct Criticality {
    pub breaking_changes: usize,
    pub new_features: usize,
    pub fixes: usize,
}

impl Criticality {
    pub fn from_counts(breaking: usize, features: usize, fixes: usize) -> Self {
        Self {
            breaking_changes: breaking,
            new_features: features,
            fixes,
        }
    }
}
