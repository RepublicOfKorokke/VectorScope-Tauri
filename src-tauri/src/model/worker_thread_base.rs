use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct Worker {
    pub keep_alive: Arc<AtomicBool>,
}

pub trait WorkerTrait {
    fn run(&self, app_handle: tauri::AppHandle);
    fn stop(&self);
}

impl Worker {
    pub fn new() -> Self {
        Self {
            keep_alive: Arc::new(AtomicBool::new(true)),
        }
    }
}
