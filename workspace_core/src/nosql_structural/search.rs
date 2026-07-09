use crate::schema::ProjectDocument;

pub trait DocumentMatcher {
    fn matches(&self, field: &str, value: &str) -> bool;
}

impl DocumentMatcher for ProjectDocument {
    fn matches(&self, field: &str, value: &str) -> bool {
        match field {
            "name" => self.name == value,
            "status" => self.status.to_string() == value,
            "tags" => self.tags.as_ref()
                .map(|tags| tags.contains(&value.to_string()))
                .unwrap_or(false),
            _ => false,
        }
    }
}
