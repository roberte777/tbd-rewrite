// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tbd_rewrite::{commands, Terminal};
use tokio::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::start_term])
        .manage(Arc::new(Terminal(Mutex::new(None))))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
