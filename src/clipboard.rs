use std::{sync::Arc, thread, time::Duration};

use arboard::Clipboard;
use tokio::sync::broadcast;

pub fn start_clipboard_listener(tx: Arc<broadcast::Sender<String>>) {
    tokio::task::spawn_blocking(move || {
        let mut clipboard = match Clipboard::new() {
            Ok(cb) => cb,
            Err(e) => {
                eprintln!("[ERR] Failed to init system clipboard: {}", e);
                return;
            }
        };

        let mut last_txt = String::new();

        loop {
            if let Ok(cur_txt) = clipboard.get_text() {
                if !cur_txt.is_empty() && cur_txt != last_txt {
                    last_txt = cur_txt.clone();
                    tx.send(cur_txt).ok().unwrap_or_else(|| {
                        eprintln!("[WARN] Could not send current clipboard contents to server");
                        0
                    });
                }
            }

            thread::sleep(Duration::from_millis(400));
        }
    });
}

pub fn update_clipboard_text(text: String) {
    tokio::task::spawn_blocking(move || {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Err(e) = clipboard.set_text(text) {
                eprintln!("[ERR] Failed to set value to clipboard");
            }
        }
    });
}
