use std::{borrow::Cow, sync::Arc};

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use rust_embed::RustEmbed;
use tokio::sync::broadcast;

use crate::ws::ws_handler;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

pub async fn run_server(
    port: u16,
    tx: Arc<broadcast::Sender<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let tx_for_ws = Arc::clone(&tx);

    let app = Router::new()
        .route("/ws", get(move |ws| ws_handler(ws, tx_for_ws)))
        .route("/", get(serve_index));

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn serve_index() -> impl IntoResponse {
    match Assets::get("index.html") {
        Some(content) => Html(content.data),
        None => Html(Cow::Borrowed("404: index.html file missing".as_bytes())),
    }
}
