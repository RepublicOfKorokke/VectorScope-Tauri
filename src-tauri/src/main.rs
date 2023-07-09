// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;
mod model;
use crate::message::payload::Payload;
use crate::model::graph_plotter;
use crate::model::mouse_info;
use crate::model::screenshot_capture;
use crate::model::vector_scope_thread;
use crate::model::worker_thread;
use crate::model::worker_thread::WorkerTrait;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

const TRAY_QUIT: &str = "QUIT";
const TRAY_VECTOR_SCOPE: &str = "VECTOR_SCOPE";

static THREAD_VECTOR_SCOPE: Lazy<RwLock<worker_thread::Worker>> =
    Lazy::new(|| RwLock::new(vector_scope_thread::create_vector_scope_thread()));

#[tauri::command]
fn get_mouse_position() -> (i32, i32) {
    return mouse_info::get_mouse_position();
}

#[tauri::command]
fn print_log(text: &str) {
    println!("{}", text)
}

#[tauri::command]
fn create_capture_window(handle: tauri::AppHandle) {
    let _capture_window = match tauri::WindowBuilder::new(
        &handle,
        "capture_window", /* the unique window label */
        tauri::WindowUrl::App("index.html".into()),
    )
    .build()
    {
        Err(_err) => {
            println!("Failed to create capture window")
        }
        Ok(_ok) => {}
    };
}

#[tauri::command]
fn get_vector_scope_image_as_payload() -> Payload {
    return vector_scope_thread::get_vector_scope_image_as_payload();
}

#[tauri::command]
fn start_emit_capture_result(window: tauri::Window) {
    println!("emit_capture_result");
    THREAD_VECTOR_SCOPE
        .try_read()
        .expect("Failed to get THREAD_VECTOR_SCOPE")
        .run(window);
}

#[tauri::command]
fn stop_emit_capture_result() {
    THREAD_VECTOR_SCOPE
        .try_read()
        .expect("Failed to get THREAD_VECTOR_SCOPE")
        .stop();
}

fn main() {
    let quit = CustomMenuItem::new(TRAY_QUIT, "Quit");
    let vector_scope = CustomMenuItem::new(TRAY_VECTOR_SCOPE, "Vector Scope");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(vector_scope);

    tauri::Builder::default()
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
                    create_capture_window(app.app_handle());
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            print_log,
            get_mouse_position,
            create_capture_window,
            get_vector_scope_image_as_payload,
            start_emit_capture_result,
            stop_emit_capture_result,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
