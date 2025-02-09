use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::task;
use winapi::shared::windef::HWND;

use crate::overlay::create_overlay;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ThreadType {
    GameOverlay,
}

#[derive(Debug)]
pub struct ThreadHandler {
    threads: Arc<Mutex<HashMap<ThreadType, task::JoinHandle<()>>>>,
}

impl ThreadHandler {
    pub fn new() -> Self {
        Self {
            threads: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn start_thread(&self, game_window: HWND, thread_type: ThreadType) {
    let threads = Arc::clone(&self.threads);
    let hwnd_value = game_window as usize; // Convert HWND to usize

    let handle = tokio::task::spawn_blocking(move || {
        let game_hwnd = hwnd_value as HWND; // Convert usize back to HWND

        match thread_type {
            ThreadType::GameOverlay => {
                create_overlay(game_hwnd); // Now using valid HWND safely
            }
        }
    });

    let mut map = threads.lock().await;
    map.insert(thread_type, handle);
}    
    pub async fn stop_thread(&self, thread_type: &ThreadType) {
        let mut threads = self.threads.lock().await;
        if let Some(handle) = threads.remove(thread_type) {
            handle.abort();
        }
    }

}


