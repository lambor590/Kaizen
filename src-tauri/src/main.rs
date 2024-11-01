#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod windows_api;

use tauri::{AppHandle, Manager};
use windows_api::WindowsAPI;

#[tauri::command]
async fn close_splash(app: AppHandle) -> Result<(), tauri::Error> {
    if let Some(splash) = app.get_webview_window("splash") {
        splash.close()?;
        app.get_webview_window("main").unwrap().show()?;
    }
    Ok(())
}

#[tauri::command]
async fn check_admin(ask: bool) -> bool {
    ask && WindowsAPI::restart_as_admin() || WindowsAPI::check_admin()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            close_splash,
            check_admin,
            tools::cleaner::run_cleaner,
            tools::downloader::run_downloader,
            tools::downloader::get_video_data,
            tools::downloader::check_downloader_deps,
            tools::downloader::install_downloader_deps
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
