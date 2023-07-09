use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct Worker {
    pub to_stop: Arc<AtomicBool>,
}

pub trait WorkerTrait {
    fn run(&self, window: tauri::Window);
    fn stop(&self);
}

impl Worker {
    pub fn new() -> Self {
        Self {
            to_stop: Arc::new(AtomicBool::new(false)),
        }
    }
}
