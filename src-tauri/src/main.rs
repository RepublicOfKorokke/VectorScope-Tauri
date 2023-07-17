// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_view_model;
mod message;
mod model;
use crate::model::graph_plotter;
use crate::model::mouse_info;
use crate::model::screenshot_capture;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

const WINDOW_LABEL_VECTOR_SCOPE: &str = "window_vector_scope";
const WINDOW_LABEL_WAVEFORM: &str = "window_waveform";
const WINDOW_LABEL_CAPTURE_AREA_SETTING: &str = "window_capture_area_setting";

const TRAY_QUIT: &str = "QUIT";
const TRAY_VECTOR_SCOPE: &str = "VECTOR_SCOPE";
const TRAY_WAVEFORM: &str = "WAVEFORM";
const TRAY_CAPTURE_AREA_SETTING: &str = "CAPTURE_AREA_SETTING";

#[tauri::command]
fn get_mouse_position() -> (i32, i32) {
    return mouse_info::get_mouse_position();
}

#[tauri::command]
fn print_log(_text: &str) {
    #[cfg(debug_assertions)]
    println!("{}", _text)
}

fn create_vector_scope_window(app_handle: tauri::AppHandle) {
    let _vector_scope_window = match tauri::WindowBuilder::new(
        &app_handle,
        WINDOW_LABEL_VECTOR_SCOPE,
        tauri::WindowUrl::App("index.html".into()),
    )
    .build()
    {
        Err(_err) => {
            #[cfg(debug_assertions)]
            println!("{_err}");
            let _ = app_handle
                .get_window(WINDOW_LABEL_VECTOR_SCOPE)
                .expect("vector scope window not found")
                .set_focus();
        }
        Ok(_ok) => {}
    };
}

fn create_waveform_window(app_handle: tauri::AppHandle) {
    let _vector_scope_window = match tauri::WindowBuilder::new(
        &app_handle,
        WINDOW_LABEL_WAVEFORM,
        tauri::WindowUrl::App("waveform.html".into()),
    )
    .build()
    {
        Err(_err) => {
            #[cfg(debug_assertions)]
            println!("{_err}");
            let _ = app_handle
                .get_window(WINDOW_LABEL_WAVEFORM)
                .expect("waveform window not found")
                .set_focus();
        }
        Ok(_ok) => {}
    };
}

#[tauri::command]
fn create_capture_area_setting_window(app_handle: tauri::AppHandle) {
    let _capture_area_setting_window = match tauri::WindowBuilder::new(
        &app_handle,
        WINDOW_LABEL_CAPTURE_AREA_SETTING,
        tauri::WindowUrl::App("capture_area_setting_window.html".into()),
    )
    .build()
    {
        Err(_err) => {
            #[cfg(debug_assertions)]
            println!("{_err}");
            let _ = app_handle
                .get_window(WINDOW_LABEL_CAPTURE_AREA_SETTING)
                .expect("capture area setting window not found")
                .set_focus();
        }
        Ok(_ok) => {}
    };
}

fn main() {
    let quit = CustomMenuItem::new(TRAY_QUIT, "Quit");
    let vector_scope = CustomMenuItem::new(TRAY_VECTOR_SCOPE, "Vector Scope");
    let waveform = CustomMenuItem::new(TRAY_WAVEFORM, "Waveform");
    let capture_area_setting =
        CustomMenuItem::new(TRAY_CAPTURE_AREA_SETTING, "Capture area setting");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(vector_scope)
        .add_item(waveform)
        .add_item(capture_area_setting);

    let mut app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                #[cfg(debug_assertions)]
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                #[cfg(debug_assertions)]
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                #[cfg(debug_assertions)]
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                TRAY_QUIT => {
                    std::process::exit(0);
                }
                TRAY_VECTOR_SCOPE => {
                    #[cfg(debug_assertions)]
                    println!("system tray VECTOR_SCOPE click");
                    create_vector_scope_window(app.app_handle());
                }
                TRAY_WAVEFORM => {
                    #[cfg(debug_assertions)]
                    println!("system tray WAVEFORM click");
                    create_waveform_window(app.app_handle());
                }
                TRAY_CAPTURE_AREA_SETTING => {
                    #[cfg(debug_assertions)]
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
            main_view_model::initialize_capture_area,
            main_view_model::set_capture_area,
            main_view_model::set_is_vector_scope_window_open,
            main_view_model::set_is_waveform_window_open,
            main_view_model::set_manual_mode,
            main_view_model::one_shot_emit,
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
