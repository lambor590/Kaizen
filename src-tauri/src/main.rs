#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![close_splash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}