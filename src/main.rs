use std::sync::Arc;

use tokio::sync::broadcast;

use crate::{
    clipboard::start_clipboard_listener, network::get_local_ip_and_print_qr, server::run_server,
};

mod clipboard;
mod network;
mod server;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 8080;

    let (tx, _rx) = broadcast::channel::<String>(100);
    let tx = Arc::new(tx);

    // Start the clipboard listener
    start_clipboard_listener(Arc::clone(&tx));

    // Display URL and QR code
    if let Err(e) = get_local_ip_and_print_qr(port) {
        eprintln!("[WARN] Could not fetch local IP: {}", e);
    }

    // Start server
    run_server(port, Arc::clone(&tx)).await?;
    Ok(())
}
