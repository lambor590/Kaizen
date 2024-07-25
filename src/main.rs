mod logger;
mod updater;

use crossterm::{execute, terminal::SetTitle};
use logger::Logger;
use updater::Updater;

fn main() {
    execute!(
        std::io::stdout(),
        SetTitle(format!("Ghost Toolkit - v{}", env!("CARGO_PKG_VERSION")))
    )
    .unwrap();
    Logger::info("Primera versión pública de Ghost Toolkit hecho en Rust");

    if let Err(e) = Updater::check_updates() {
        Logger::error(format!("Error al comprobar actualizaciones: {}", &e).as_str());
    }

    std::io::stdin().read_line(&mut String::new()).unwrap();
}
