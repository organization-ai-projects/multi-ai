use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

mod rest;
mod websocket;

pub use rest::create_router;
pub(crate) use websocket::handle_connection;
