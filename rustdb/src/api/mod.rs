use std::path::PathBuf;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;

mod rest;
mod websocket;

pub use rest::create_router;
pub use websocket::handle_connection;
