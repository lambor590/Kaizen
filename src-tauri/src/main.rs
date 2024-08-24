#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod windows_api;

use tauri::{Manager, Window};
use windows_api::WindowsAPI;

#[tauri::command]
async fn close_splash(window: Window) {
    window
        .get_window("splash")
        .expect("no window labeled 'splash' found")
        .close()
        .unwrap();

    window
        .get_window("main")
        .expect("no window labeled 'main' found")
        .show()
        .unwrap();
}

#[tauri::command]
async fn check_admin(ask: bool) -> bool {
    if ask {
        WindowsAPI::restart_as_admin()
    }
    WindowsAPI::check_admin()
}

fn main() {
    tauri::Builder::default()
        .plugin(tools::init())
        .invoke_handler(tauri::generate_handler![close_splash, check_admin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
