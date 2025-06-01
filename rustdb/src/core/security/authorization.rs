use super::authentication::User;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Permission {
    resource: String,
    action: Action,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Action {
    Read,
    Write,
    Delete,
    Admin,
}

pub struct RbacManager {
    role_permissions: HashMap<String, HashSet<Permission>>,
}

impl RbacManager {
    pub fn new() -> Self {
        Self {
            role_permissions: HashMap::new(),
        }
    }

    pub fn can_access(&self, user: &User, resource: &str, action: Action) -> bool {
        user.roles.iter().any(|role| {
            if let Some(permissions) = self.role_permissions.get(role) {
                permissions.contains(&Permission {
                    resource: resource.to_string(),
                    action: action.clone(),
                })
            } else {
                false
            }
        })
    }

    pub fn add_role_permission(&mut self, role: &str, permission: Permission) {
        self.role_permissions
            .entry(role.to_string())
            .or_insert_with(HashSet::new)
            .insert(permission);
    }
}
