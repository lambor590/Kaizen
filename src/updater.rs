use crate::Logger;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    env::{consts, current_exe},
    fs::{remove_file, write},
    io::stdout,
    process,
};

pub struct Updater;

impl Updater {
    pub fn check_updates() -> Result<(), Box<dyn std::error::Error>> {
        let current_version: &str = env!("CARGO_PKG_VERSION");
        let response: serde_json::Value = reqwest::blocking::Client::new()
            .get("https://api.github.com/repos/lambor590/Ghost-Toolkit/tags")
            .header("User-Agent", "Ghost-Toolkit")
            .send()?
            .error_for_status()?
            .json()?;

        let latest_version: &str = &response[0]["name"].as_str().unwrap();

        if &current_version == &latest_version {
            Logger::info("No hay actualizaciones.");
            return Ok(());
        }

        Logger::warn("Hay una nueva versión disponible. Descargando...");

        let arch_extension: &str = match (consts::OS, consts::ARCH) {
            ("windows", "x86_64") => "windows-msvc.exe",
            ("linux", "x86_64") => "linux-x64",
            ("linux", "aarch64") => "linux-arm64",
            ("macos", "x86_64") => "macos-x64",
            ("macos", "aarch64") => "macos-arm",
            _ => panic!("Combinación de sistema operativo y arquitectura no soportada"),
        };

        let binary = reqwest::blocking::get(format!(
            "https://github.com/lambor590/Ghost-Toolkit/releases/download/{}/ghost-toolkit-{}",
            &latest_version, &arch_extension
        ))?
        .bytes()?;

        let new_bin: String = format!(
            "{}/Ghost-Toolkit{}",
            std::env::temp_dir().to_str().unwrap(),
            consts::EXE_SUFFIX
        );

        write(&new_bin, &binary)?;

        Logger::info("Descarga completada. Actualizando en caliente...");
        std::thread::sleep(std::time::Duration::from_secs(1));

        self_replace::self_replace(&new_bin)?;
        remove_file(&new_bin)?;

        execute!(stdout(), Clear(ClearType::FromCursorUp)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();

        process::Command::new(current_exe().unwrap())
            .spawn()
            .expect("Error al abrir la nueva instancia");

        process::exit(0);
    }
}
