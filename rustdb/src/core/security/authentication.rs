use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::super::error::Result;

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
    roles: Vec<String>,
}

pub struct AuthManager {
    config: Config<'static>,
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_user(&self, username: &str, password: &str, roles: Vec<String>) -> Result<User> {
        let salt = Uuid::new_v4().as_bytes();
        let hash = argon2::hash_encoded(
            password.as_bytes(),
            salt,
            &self.config
        )?;

        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            password_hash: hash,
            roles,
        };

        self.users.write().await.insert(username.to_string(), user.clone());
        Ok(user)
    }

    pub async fn verify(&self, username: &str, password: &str) -> Result<bool> {
        let users = self.users.read().await;
        if let Some(user) = users.get(username) {
            Ok(argon2::verify_encoded(&user.password_hash, password.as_bytes())?)
        } else {
            Ok(false)
        }
    }
}
