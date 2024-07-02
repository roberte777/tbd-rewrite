use std::sync::Arc;

use alacritty_terminal::event::Event;
use tauri::{Manager, State};

use crate::{
    backend::{self, Size},
    Terminal,
};

#[tauri::command]
pub async fn start_term(
    term: State<'_, Arc<Terminal>>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let settings = backend::settings::BackendSettings::default();
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(100);
    let backend = backend::Backend::new(
        0,
        event_tx,
        settings,
        Size {
            width: 0.,
            height: 0.,
        },
    )
    .map_err(|e| e.to_string())?;
    // if term backend is not none, throw error. Else, set backend
    {
        let mut term = term.0.lock().await;
        if term.is_some() {
            return Err("Terminal already running".to_string());
        }
        term.replace(backend);
    }

    // spwn a task to handle the events
    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            match event {
                Event::Title(title) => {
                    println!("title: {}", title);
                }
                Event::Exit => {
                    println!("exit");
                }
                Event::ChildExit(code) => {
                    println!("Exit with code: {}", code);
                }
                e => {
                    println!("unhandled event: {:?}", e);
                }
            }
        }
    });

    let term = (*term).clone();
    tokio::spawn(async move {
        // 60 fps, send grid to frontend
        loop {
            let mut backend = term.0.lock().await;
            let backend = backend.as_mut();
            match backend {
                Some(backend) => {
                    backend.sync();
                    let content = backend.renderable_content();
                    handle.emit_all("grid", content).unwrap();
                }
                None => {
                    break;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
        }
    });

    Ok(())
}
