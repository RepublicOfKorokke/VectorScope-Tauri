use crate::graph_plotter;
use crate::message;
use crate::model::worker_thread::Worker;
use crate::model::worker_thread::WorkerTrait;
use crate::screenshot_capture;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const PREFIX_DATA_URI: &str = "data:image/png;base64,";

impl WorkerTrait for Worker {
    fn run(&self, window: tauri::Window) {
        let to_stop = Arc::clone(&self.to_stop);
        to_stop.store(false, Ordering::Relaxed);
        thread::spawn(move || loop {
            if to_stop.load(Ordering::Relaxed) {
                break;
            }
            let screenshot = screenshot_capture::capture_entire_sreen();
            const CUSTOM_ENGINE: engine::GeneralPurpose =
                engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD);
            let vector_scope_image =
                graph_plotter::draw_vectorscope(screenshot).expect("Failed to draw vector scope");
            let base64 = CUSTOM_ENGINE.encode(vector_scope_image);

            let data_uri = PREFIX_DATA_URI.to_string() + &base64;
            window
                .emit(
                    "event-capture-screen",
                    message::payload::Payload::new(data_uri),
                )
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    }
    fn stop(&self) {
        println!("worker1 try to stop");
        self.to_stop.store(true, Ordering::Relaxed);
    }
}

pub fn create_vector_scope_thread() -> Worker {
    Worker::new()
}
