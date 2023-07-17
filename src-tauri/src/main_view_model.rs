use crate::graph_plotter;
use crate::message::payload::Payload;
use crate::model::worker_thread_base;
use crate::model::worker_thread_base::WorkerTrait;
use crate::screenshot_capture;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use once_cell::sync::Lazy;
use screenshots::Image;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use tauri::Manager;

const PREFIX_DATA_URI: &str = "data:image/png;base64,";
const EVENT_NAME_VECTOR_SCOPE: &str = "event-vector-scope";
const EVENT_NAME_WAVEFORM: &str = "event-waveform";

static BASE64_ENGINE: OnceLock<engine::GeneralPurpose> = OnceLock::new();

static CAPTURE_AREA_TOP_LEFT: Lazy<RwLock<(i32, i32)>> = Lazy::new(|| RwLock::new((0, 0)));
static CAPTURE_AREA_BOTTOM_RIGHT: Lazy<RwLock<(i32, i32)>> = Lazy::new(|| RwLock::new((0, 0)));
static IS_VECTOR_SCOPE_REQUIRED: Lazy<Arc<AtomicBool>> =
    Lazy::new(|| Arc::new(AtomicBool::new(false)));
static IS_WAVEFORM_REQUIRED: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
static IS_MANUAL_REFRESH_MODE_ON: Lazy<Arc<AtomicBool>> =
    Lazy::new(|| Arc::new(AtomicBool::new(false)));

static THREAD_IMAGE_PROCESS: Lazy<RwLock<ImageProcessThread>> =
    Lazy::new(|| RwLock::new(create_vector_scope_thread()));

pub struct ImageProcessThread {
    pub worker_thread: worker_thread_base::Worker,
}

impl ImageProcessThread {
    pub fn new() -> Self {
        Self {
            worker_thread: worker_thread_base::Worker::new(),
        }
    }
}

impl worker_thread_base::WorkerTrait for ImageProcessThread {
    fn run(&self, app_handle: tauri::AppHandle) {
        let keep_alive = Arc::clone(&self.worker_thread.keep_alive);
        keep_alive.store(true, Ordering::Relaxed);
        thread::spawn(move || loop {
            if !keep_alive.load(Ordering::Relaxed) {
                break;
            }
            let payload = get_graph_image_as_payload();
            if !payload.base64_vector_scope.is_empty() {
                app_handle
                    .emit_to(
                        super::WINDOW_LABEL_VECTOR_SCOPE,
                        EVENT_NAME_VECTOR_SCOPE,
                        &payload.base64_vector_scope,
                    )
                    .unwrap();
            }
            if !payload.base64_waveform.is_empty() {
                app_handle
                    .emit_to(
                        super::WINDOW_LABEL_WAVEFORM,
                        EVENT_NAME_WAVEFORM,
                        &payload.base64_waveform,
                    )
                    .unwrap();
            }
            thread::sleep(Duration::from_secs(1));
        });
    }
    fn stop(&self) {
        self.worker_thread
            .keep_alive
            .store(false, Ordering::Relaxed);
    }
}

#[tauri::command]
pub fn one_shot_emit(app_handle: tauri::AppHandle) {
    let screenshot = match is_capture_area_valid() {
        true => {
            let top_left: (i32, i32) = *CAPTURE_AREA_TOP_LEFT.try_read().unwrap();
            let bottom_right: (i32, i32) = *CAPTURE_AREA_BOTTOM_RIGHT.try_read().unwrap();
            screenshot_capture::capture_area(top_left, bottom_right)
        }
        false => screenshot_capture::capture_entire_sreen(),
    };

    #[cfg(debug_assertions)]
    println!(
        "IS_VECTOR_SCOPE_REQUIRED: {}",
        IS_VECTOR_SCOPE_REQUIRED.load(Ordering::Relaxed)
    );
    #[cfg(debug_assertions)]
    println!(
        "IS_WAVEFORM_REQUIRED: {}",
        IS_WAVEFORM_REQUIRED.load(Ordering::Relaxed)
    );

    if IS_VECTOR_SCOPE_REQUIRED.load(Ordering::Relaxed) {
        let base64_vector_scope = get_vector_scope_image_as_base64(&screenshot);
        app_handle
            .emit_to(
                super::WINDOW_LABEL_VECTOR_SCOPE,
                EVENT_NAME_VECTOR_SCOPE,
                base64_vector_scope,
            )
            .unwrap();
    }

    if IS_WAVEFORM_REQUIRED.load(Ordering::Relaxed) {
        let base64_waveform = get_waveform_image_as_base64(&screenshot);
        app_handle
            .emit_to(
                super::WINDOW_LABEL_WAVEFORM,
                EVENT_NAME_WAVEFORM,
                base64_waveform,
            )
            .unwrap();
    }
}

#[cold]
pub fn create_vector_scope_thread() -> ImageProcessThread {
    ImageProcessThread::new()
}

#[tauri::command]
pub fn initialize_capture_area() {
    let mut top_left_writer = CAPTURE_AREA_TOP_LEFT.write().unwrap();
    *top_left_writer = (0, 0);
    let mut bottom_right_writer = CAPTURE_AREA_BOTTOM_RIGHT.write().unwrap();
    *bottom_right_writer = (0, 0);
}

#[tauri::command]
pub fn set_capture_area(top_left: (i32, i32), bottom_right: (i32, i32)) {
    let mut top_left_writer = CAPTURE_AREA_TOP_LEFT.write().unwrap();
    *top_left_writer = top_left;
    let mut bottom_right_writer = CAPTURE_AREA_BOTTOM_RIGHT.write().unwrap();
    *bottom_right_writer = bottom_right;
}

#[tauri::command]
pub fn set_is_vector_scope_required(app_handle: tauri::AppHandle, state: bool) {
    if IS_VECTOR_SCOPE_REQUIRED.load(Ordering::Relaxed) != state {
        IS_VECTOR_SCOPE_REQUIRED.store(state, Ordering::Relaxed);
        check_thread_need_to_be_keep_alive(app_handle);
    }
}

#[tauri::command]
pub fn set_is_waveform_required(app_handle: tauri::AppHandle, state: bool) {
    if IS_WAVEFORM_REQUIRED.load(Ordering::Relaxed) != state {
        IS_WAVEFORM_REQUIRED.store(state, Ordering::Relaxed);
        check_thread_need_to_be_keep_alive(app_handle);
    }
}

#[tauri::command]
pub fn set_manual_mode(app_handle: tauri::AppHandle, state: bool) {
    #[cfg(debug_assertions)]
    println!("set_manual_mode: {state}");
    if IS_MANUAL_REFRESH_MODE_ON.load(Ordering::Relaxed) != state {
        IS_MANUAL_REFRESH_MODE_ON.store(state, Ordering::Relaxed);
        check_thread_need_to_be_keep_alive(app_handle);
    }
}

#[tauri::command]
pub fn get_graph_image_as_payload() -> Payload {
    let screenshot = match is_capture_area_valid() {
        true => {
            let top_left: (i32, i32) = *CAPTURE_AREA_TOP_LEFT.try_read().unwrap();
            let bottom_right: (i32, i32) = *CAPTURE_AREA_BOTTOM_RIGHT.try_read().unwrap();
            screenshot_capture::capture_area(top_left, bottom_right)
        }
        false => screenshot_capture::capture_entire_sreen(),
    };

    let mut base64_vector_scope: String = String::new();
    let mut base64_waveform: String = String::new();

    if IS_VECTOR_SCOPE_REQUIRED.load(Ordering::Relaxed) {
        base64_vector_scope = get_vector_scope_image_as_base64(&screenshot);
    }

    if IS_WAVEFORM_REQUIRED.load(Ordering::Relaxed) {
        base64_waveform = get_waveform_image_as_base64(&screenshot);
    }

    Payload::new(base64_vector_scope, base64_waveform)
}

fn get_vector_scope_image_as_base64(screenshot: &Image) -> String {
    let vector_scope_image =
        graph_plotter::draw_vector_scope(&screenshot).expect("Failed to draw vector scope");
    let base64_vector_scope = BASE64_ENGINE
        .get_or_init(init_base64_engine)
        .encode(vector_scope_image);
    PREFIX_DATA_URI.to_string() + &base64_vector_scope
}

fn get_waveform_image_as_base64(screenshot: &Image) -> String {
    let waveform_image =
        graph_plotter::draw_waveform(&screenshot).expect("Failed to draw waveform");
    let base64_waveform = BASE64_ENGINE
        .get_or_init(init_base64_engine)
        .encode(waveform_image);
    PREFIX_DATA_URI.to_string() + &base64_waveform
}

fn is_capture_area_valid() -> bool {
    let top_left = CAPTURE_AREA_TOP_LEFT.try_read();
    let bottom_right = CAPTURE_AREA_BOTTOM_RIGHT.try_read();

    if top_left.is_err() || bottom_right.is_err() {
        return false;
    }

    if *top_left.unwrap() == *bottom_right.unwrap() {
        false
    } else {
        true
    }
}

fn check_thread_need_to_be_keep_alive(app_handle: tauri::AppHandle) {
    if (IS_VECTOR_SCOPE_REQUIRED.load(Ordering::Relaxed)
        || IS_WAVEFORM_REQUIRED.load(Ordering::Relaxed))
        && !IS_MANUAL_REFRESH_MODE_ON.load(Ordering::Relaxed)
    {
        if !THREAD_IMAGE_PROCESS
            .try_read()
            .expect("Failed to read thread")
            .worker_thread
            .keep_alive
            .load(Ordering::Relaxed)
        {
            #[cfg(debug_assertions)]
            println!("Thread: Start");
            THREAD_IMAGE_PROCESS
                .try_read()
                .expect("Failed to read thread")
                .run(app_handle)
        } else {
            #[cfg(debug_assertions)]
            println!("Thread: Already started");
        }
    } else {
        if THREAD_IMAGE_PROCESS
            .try_read()
            .expect("Failed to read thread")
            .worker_thread
            .keep_alive
            .load(Ordering::Relaxed)
        {
            #[cfg(debug_assertions)]
            println!("Thread: Stop");
            THREAD_IMAGE_PROCESS
                .try_read()
                .expect("Failed to read thread")
                .stop()
        }
    }
}

#[cold]
fn init_base64_engine() -> engine::GeneralPurpose {
    engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD)
}
