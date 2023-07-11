use crate::graph_plotter;
use crate::message::payload::Payload;
use crate::model::worker_thread;
use crate::screenshot_capture;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use once_cell::sync::Lazy;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

const PREFIX_DATA_URI: &str = "data:image/png;base64,";
const EVENT_NAME: &str = "event-vector-scope";

static CAPTURE_AREA_TOP_LEFT: Lazy<RwLock<(i32, i32)>> = Lazy::new(|| RwLock::new((0, 0)));
static CAPTURE_AREA_BOTTOM_RIGHT: Lazy<RwLock<(i32, i32)>> = Lazy::new(|| RwLock::new((0, 0)));
static BASE64_ENGINE: OnceLock<engine::GeneralPurpose> = OnceLock::new();

pub struct VectorScopeWorker {
    pub worker_thread: worker_thread::Worker,
}

impl VectorScopeWorker {
    pub fn new() -> Self {
        Self {
            worker_thread: worker_thread::Worker::new(),
        }
    }
}

impl worker_thread::WorkerTrait for VectorScopeWorker {
    fn run(&self, window: tauri::Window) {
        let keep_alive = Arc::clone(&self.worker_thread.keep_alive);
        keep_alive.store(true, Ordering::Relaxed);
        thread::spawn(move || loop {
            if !keep_alive.load(Ordering::Relaxed) {
                break;
            }
            let payload = get_vector_scope_image_as_payload();
            window.emit(EVENT_NAME, payload).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    }
    fn stop(&self) {
        self.worker_thread
            .keep_alive
            .store(false, Ordering::Relaxed);
    }
}

pub fn create_vector_scope_thread() -> VectorScopeWorker {
    VectorScopeWorker::new()
}

pub fn init_capture_area() {
    let mut top_left_writer = CAPTURE_AREA_TOP_LEFT.write().unwrap();
    *top_left_writer = (0, 0);
    let mut bottom_right_writer = CAPTURE_AREA_BOTTOM_RIGHT.write().unwrap();
    *bottom_right_writer = (0, 0);
}

pub fn set_capture_area(top_left: (i32, i32), bottom_right: (i32, i32)) {
    let mut top_left_writer = CAPTURE_AREA_TOP_LEFT.write().unwrap();
    *top_left_writer = top_left;
    let mut bottom_right_writer = CAPTURE_AREA_BOTTOM_RIGHT.write().unwrap();
    *bottom_right_writer = bottom_right;
}

pub fn get_vector_scope_image_as_payload() -> Payload {
    let vector_scope_image = match is_capture_area_valid() {
        true => create_vector_scope_image_from_area(),
        false => create_vector_scope_image(),
    };

    let base64 = BASE64_ENGINE
        .get_or_init(init_base64_engine)
        .encode(vector_scope_image);
    let data_uri = PREFIX_DATA_URI.to_string() + &base64;
    Payload::new(data_uri)
}

fn is_capture_area_valid() -> bool {
    let top_left = CAPTURE_AREA_TOP_LEFT.try_read();
    let bottom_right = CAPTURE_AREA_BOTTOM_RIGHT.try_read();

    if top_left.is_err() || bottom_right.is_err() {
        return false;
    }

    if *top_left.unwrap() == *bottom_right.unwrap() {
        println!("capture area is not set");
        false
    } else {
        println!("capture area is set");
        true
    }
}

fn create_vector_scope_image() -> Vec<u8> {
    let screenshot = screenshot_capture::capture_entire_sreen();
    graph_plotter::draw_vectorscope(screenshot).expect("Failed to draw vector scope")
}

fn create_vector_scope_image_from_area() -> Vec<u8> {
    let top_left: (i32, i32) = *CAPTURE_AREA_TOP_LEFT.try_read().unwrap();
    let bottom_right: (i32, i32) = *CAPTURE_AREA_BOTTOM_RIGHT.try_read().unwrap();

    let screenshot = screenshot_capture::capture_area(top_left, bottom_right);
    graph_plotter::draw_vectorscope(screenshot).expect("Failed to draw vector scope")
}

fn init_base64_engine() -> engine::GeneralPurpose {
    engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD)
}
