use std::sync::Arc;

use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use tokio::sync::broadcast;

use crate::clipboard;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    tx: Arc<broadcast::Sender<String>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: Arc<broadcast::Sender<String>>) {
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            // Desktop copied (rust binary) -> send to mobile/other devices (server)
            Ok(desktop_txt) = rx.recv() => {
                if socket.send(Message::Text(desktop_txt.into())).await.is_err() {
                    break;
                }
            }
            // Mobile (server) UI sent content -> update primary device clipboard
            Some(Ok(msg)) = socket.recv() => {
                if let Message::Text(mobile_txt) = msg {
                    clipboard::update_clipboard_text(mobile_txt.to_string());
                }
            }
            else => break,
        }
    }
}
