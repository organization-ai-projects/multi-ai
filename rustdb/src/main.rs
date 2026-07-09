mod api;
mod core;

use crate::server::RustDbServer;
use tokio;

#[tokio::main]
async fn main() {
    let server = RustDbServer::new("127.0.0.1:8080");
    server.start().await;
}
