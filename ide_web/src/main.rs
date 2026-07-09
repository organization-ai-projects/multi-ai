use ide_web::{create_router, websocket};
use axum::serve;
use tokio::sync::broadcast;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(100); // Canal pour suggestions en temps réel
    let app = create_router(tx);

    println!("🚀 IDE Web lancé sur http://localhost:3000");
    
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
