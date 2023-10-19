// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keyboard;
mod gsmtc;

use serde::Serialize;
use tauri::{Manager, App};
use tokio::sync::mpsc::unbounded_channel;

#[derive(Debug, Serialize, Clone)]
pub enum UpdateType {
    GsmtcUpdate,
    KeyboardUpdate

}

fn init_background_runners(app: &mut App) {
    
    let (mut kb_tx, mut rx) = unbounded_channel::<UpdateType>();

    let mut gsmtc_tx = kb_tx.clone();

    tokio::spawn(async move {
        keyboard::init(&mut kb_tx).await;
    });

    tokio::spawn(async move {   
        gsmtc::init(&mut gsmtc_tx).await;
    });

    let app_handle = app.app_handle();

    tokio::spawn(async move {
        loop {
            let event = rx.try_recv();
            if event.is_ok() {
                let event = event.unwrap();
                app_handle.emit_all("event", event).ok();
            }
        }
    });
}

#[tokio::main]
async fn main() {

    tauri::Builder::default()
        .setup(|app: &mut App| {

            let mut app = app;

            init_background_runners(&mut app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
