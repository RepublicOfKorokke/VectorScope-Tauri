// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_view_model;
mod message;
mod model;
use crate::message::payload::Payload;
use crate::model::graph_plotter;
use crate::model::mouse_info;
use crate::model::screenshot_capture;
use crate::model::worker_thread_base::WorkerTrait;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

const WINDOW_LABEL_VECTOR_SCOPE: &str = "window_vector_scope";
const WINDOW_LABEL_CAPTURE_AREA_SETTING: &str = "window_capture_area_setting";

const TRAY_QUIT: &str = "QUIT";
const TRAY_VECTOR_SCOPE: &str = "VECTOR_SCOPE";
const TRAY_CAPTURE_AREA_SETTING: &str = "CAPTURE_AREA_SETTING";

static THREAD_VECTOR_SCOPE: Lazy<RwLock<main_view_model::VectorScopeWorker>> =
    Lazy::new(|| RwLock::new(main_view_model::create_vector_scope_thread()));

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
        WINDOW_LABEL_VECTOR_SCOPE,
        tauri::WindowUrl::App("index.html".into()),
    )
    .build()
    {
        Err(err) => {
            println!("{err}");
            let _ = handle
                .get_window(WINDOW_LABEL_VECTOR_SCOPE)
                .expect("vector scope window not found")
                .set_focus();
        }
        Ok(_ok) => {}
    };
}

#[tauri::command]
fn create_capture_area_setting_window(handle: tauri::AppHandle) {
    let _capture_area_setting_window = match tauri::WindowBuilder::new(
        &handle,
        WINDOW_LABEL_CAPTURE_AREA_SETTING,
        tauri::WindowUrl::App("capture_area_setting_window.html".into()),
    )
    .build()
    {
        Err(err) => {
            println!("{err}");
            let _ = handle
                .get_window(WINDOW_LABEL_CAPTURE_AREA_SETTING)
                .expect("capture area setting window not found")
                .set_focus();
        }
        Ok(_ok) => {}
    };
}

#[tauri::command]
fn init_capture_area() {
    main_view_model::init_capture_area();
}

#[tauri::command]
fn set_capture_area(top_left: (i32, i32), bottom_right: (i32, i32)) {
    main_view_model::set_capture_area(top_left, bottom_right);
}

#[tauri::command]
fn get_vector_scope_image_as_payload() -> Payload {
    return main_view_model::get_vector_scope_image_as_payload();
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
