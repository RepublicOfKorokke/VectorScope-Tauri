// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;
mod model;
use crate::message::payload::Payload;
use crate::model::graph_plotter;
use crate::model::mouse_info;
use crate::model::screenshot_capture;
use crate::model::vector_scope;
use crate::model::worker_thread::WorkerTrait;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

const TRAY_QUIT: &str = "QUIT";
const TRAY_VECTOR_SCOPE: &str = "VECTOR_SCOPE";
const TRAY_CAPTURE_AREA_SETTING: &str = "CAPTURE_AREA_SETTING";

static THREAD_VECTOR_SCOPE: Lazy<RwLock<vector_scope::VectorScopeWorker>> =
    Lazy::new(|| RwLock::new(vector_scope::create_vector_scope_thread()));

#[tauri::command]
fn get_mouse_position() -> (i32, i32) {
    return mouse_info::get_mouse_position();
}

#[tauri::command]
fn print_log(text: &str) {
    println!("{}", text)
}

#[tauri::command]
fn create_vector_scope_window(handle: tauri::AppHandle) {
    let _vector_scope_window = match tauri::WindowBuilder::new(
        &handle,
        "window_vector_scope", /* the unique window label */
        tauri::WindowUrl::App("index.html".into()),
    )
    .build()
    {
        Err(_err) => {
            println!("Failed to vector scope window")
        }
        Ok(_ok) => {}
    };
}

#[tauri::command]
fn create_capture_area_setting_window(handle: tauri::AppHandle) {
    let _capture_area_setting_window = match tauri::WindowBuilder::new(
        &handle,
        "window_capture_area_setting", /* the unique window label */
        tauri::WindowUrl::App("capture_area_setting_window.html".into()),
    )
    .build()
    {
        Err(_err) => {
            println!("Failed to create capture area setting window")
        }
        Ok(_ok) => {}
    };
}

#[tauri::command]
fn init_capture_area() {
    vector_scope::init_capture_area();
}

#[tauri::command]
fn set_capture_area(top_left: (i32, i32), bottom_right: (i32, i32)) {
    vector_scope::set_capture_area(top_left, bottom_right);
}

#[tauri::command]
fn get_vector_scope_image_as_payload() -> Payload {
    return vector_scope::get_vector_scope_image_as_payload();
}

#[tauri::command]
fn start_emit_vector_scope_image_as_payload(window: tauri::Window) {
    THREAD_VECTOR_SCOPE
        .try_read()
        .expect("Failed to get THREAD_VECTOR_SCOPE")
        .run(window);
}

#[tauri::command]
fn stop_emit_vector_scope_image_as_payload() {
    THREAD_VECTOR_SCOPE
        .try_read()
        .expect("Failed to get THREAD_VECTOR_SCOPE")
        .stop();
}

fn main() {
    let quit = CustomMenuItem::new(TRAY_QUIT, "Quit");
    let vector_scope = CustomMenuItem::new(TRAY_VECTOR_SCOPE, "Vector Scope");
    let capture_area_setting =
        CustomMenuItem::new(TRAY_CAPTURE_AREA_SETTING, "Capture area setting");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(vector_scope)
        .add_item(capture_area_setting);

    let mut app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                TRAY_QUIT => {
                    std::process::exit(0);
                }
                TRAY_VECTOR_SCOPE => {
                    println!("system tray VECTOR_SCOPE click");
                    create_vector_scope_window(app.app_handle());
                }
                TRAY_CAPTURE_AREA_SETTING => {
                    println!("system tray CAPTURE_AREA_SETTING click");
                    create_capture_area_setting_window(app.app_handle());
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            print_log,
            get_mouse_position,
            create_vector_scope_window,
            init_capture_area,
            set_capture_area,
            get_vector_scope_image_as_payload,
            start_emit_vector_scope_image_as_payload,
            stop_emit_vector_scope_image_as_payload,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
