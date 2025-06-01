use tokio;
use rustdb::RustDb;

#[tokio::main]
async fn main() {
    let db = RustDb::new();
    
    // Service qui écoute à la fois:
    // - REST API
    // - WebSocket
    // - API Rust native
    db.serve().await;
}
