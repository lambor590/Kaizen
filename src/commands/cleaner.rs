use crate::{console::Console, logger::Logger};

pub struct Cleaner;

impl Cleaner {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir: std::path::PathBuf = std::env::temp_dir();
        let mut total_files: u64 = 0;
        let mut deleted_files: i32 = 0;
        let mut freed_space: u64 = 0;
        let mut progress_bar: Option<indicatif::ProgressBar> = None;

        if let Ok(entries) = std::fs::read_dir(&temp_dir) {
            total_files = entries.count() as u64;
            progress_bar = Some(
                indicatif::ProgressBar::new(total_files).with_message("Limpiando archivos..."),
            );
        }

        if let Ok(entries) = std::fs::read_dir(&temp_dir) {
            for entry in entries.flatten() {
                let path: &std::path::PathBuf = &entry.path();
                let result: Result<(), std::io::Error> = if path.is_file() {
                    if let Ok(metadata) = std::fs::metadata(path) {
                        if std::fs::remove_file(path).is_ok() {
                            freed_space += metadata.len()
                        }
                    };
                    Ok(())
                } else if path.is_dir() {
                    let dir_size = Self::get_dir_size(path);

                    if std::fs::remove_dir_all(path).is_ok() {
                        freed_space += dir_size
                    }
                    Ok(())
                } else {
                    Ok(())
                };

                if result.is_ok() {
                    deleted_files += 1;
                }

                if let Some(bar) = &progress_bar {
                    bar.inc(1);
                }
            }
        }

        Console::clear();

        let freed_space_mb = freed_space as f64 / 1_048_576.0;
        Logger::warn("Algunos archivos no se pueden eliminar al estar en uso por otros programas.");
        Logger::info(&format!(
            "Archivos eliminados: {}/{}",
            deleted_files, total_files
        ));
        Logger::info(&format!("Espacio liberado: {:.2} MB", freed_space_mb));

        Ok(())
    }

    fn get_dir_size(path: &std::path::PathBuf) -> u64 {
        let mut size = 0;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        size += metadata.len();
                    }
                } else if path.is_dir() {
                    size += Self::get_dir_size(&path);
                }
            }
        }
        size
    }
}
