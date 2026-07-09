use crate::core::Database;
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{WebSocketStream, accept_async};

pub async fn start_websocket_server(database: Database, addr: &str) {
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        let db = database.clone();
        tokio::spawn(async move {
            handle_connection(stream, db).await;
        });
    }
}

pub(crate) async fn handle_connection(stream: TcpStream, db: Database) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            let response = handle_message(&db, msg.to_string()).await;
            write.send(response.into()).await.unwrap_or_default();
        }
    }
}

async fn handle_message(db: &Database, message: String) -> String {
    match serde_json::from_str::<Value>(&message) {
        Ok(command) => {
            match command["action"].as_str() {
                Some("get_collection") => {
                    if let Some(name) = command["collection"].as_str() {
                        match db.get_collection(name).await {
                            Ok(collection) => json!({"status": "success", "data": collection}),
                            Err(e) => json!({"status": "error", "message": e.to_string()}),
                        }
                    } else {
                        json!({"status": "error", "message": "Missing collection name"})
                    }
                }
                Some("insert") => {
                    // Handle insert
                    json!({"status": "success", "message": "Document inserted"})
                }
                Some("query") => {
                    // Handle query
                    json!({"status": "success", "message": "Query executed"})
                }
                _ => json!({"status": "error", "message": "Unknown command"}),
            }
        }
        Err(e) => json!({"status": "error", "message": format!("Invalid JSON: {}", e)}),
    }
    .to_string()
}
