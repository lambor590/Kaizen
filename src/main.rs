mod commands;
mod console;
mod logger;
mod updater;
mod windows_api;

use console::Console;
use crossterm::{cursor, execute};
use logger::Logger;
use std::io::stdout;
use updater::Updater;

#[cfg(target_os = "windows")]
use windows_api::WindowsAPI;

fn main() {
    Console::set_title(&format!("Ghost Toolkit - v{}", &env!("CARGO_PKG_VERSION")));
    execute!(stdout(), &cursor::MoveToRow(7), &cursor::SavePosition).unwrap();
    Console::animate_logo();

    if let Err(e) = &Updater::check_updates() {
        Logger::error(&format!("Error al comprobar actualizaciones: {}", &e).as_str());
    }

    Console::sleep_secs(2);
    Console::clear();

    #[cfg(target_os = "windows")]
    WindowsAPI::check_admin();

    Console::menu();
}
