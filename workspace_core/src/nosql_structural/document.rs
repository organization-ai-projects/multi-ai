use crate::schema::ProjectDocument;

pub trait DocumentAccess {
    fn get_field_value(&self, field: &str) -> Option<String>;
}

impl DocumentAccess for ProjectDocument {
    fn get_field_value(&self, field: &str) -> Option<String> {
        match field {
            "name" => Some(self.name.clone()),
            "status" => Some(self.status.to_string()),
            "tags" => self.tags.as_ref().map(|t| t.join(",")),
            _ => None,
        }
    }
}
