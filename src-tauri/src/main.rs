// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod capture_screen;
mod plotter;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

const TRAY_QUIT: &str = "QUIT";
const TRAY_VECTOR_SCOPE: &str = "VECTOR_SCOPE";
const PREFIX_DATA_URI: &str = "data:image/png;base64,";

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn get_mouse_position() -> (i32, i32) {
    return capture_screen::get_mouse_position();
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
fn emit_capture_result(window: tauri::Window) {
    std::thread::spawn(move || {
        let screenshot = capture_screen::capture_entire_sreen();
        const CUSTOM_ENGINE: engine::GeneralPurpose =
            engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD);
        let vector_scope_image =
            plotter::draw_vectorscope(screenshot).expect("Failed to draw vector scope");
        let base64 = CUSTOM_ENGINE.encode(vector_scope_image);

        let data_uri = PREFIX_DATA_URI.to_string() + &base64;
        window
            .emit("event-capture-screen", Payload { message: data_uri })
            .unwrap();
    });
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
            emit_capture_result
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
