use serde_json::json;
use std::fs::{self, remove_dir_all};
use std::path::PathBuf;
use tauri::{Runtime, Window};

use super::KaizenResult;

#[tauri::command]
pub async fn run_cleaner<R: Runtime>(window: Window<R>, dirs: Vec<String>) -> KaizenResult<()> {
    let mut deleted_files: i32 = 0;
    let mut total_files: i32 = 0;
    let mut freed_space: f64 = 0.0;

    for dir in &dirs {
        let path: PathBuf = PathBuf::from(dir);

        if let Ok(entries) = fs::read_dir(&path) {
            entries
                .filter_map(Result::ok)
                .for_each(|entry: fs::DirEntry| {
                    total_files += 1;

                    let path: PathBuf = entry.path();
                    if let Ok(metadata) = path.metadata() {
                        let file_size: f64 = metadata.len() as f64;
                        let result: Result<(), std::io::Error> = if metadata.is_file() {
                            fs::remove_file(&path)
                        } else {
                            remove_dir_all(&path)
                        };

                        if result.is_ok() {
                            deleted_files += 1;
                            freed_space += file_size;
                        }

                        let mb: f64 = freed_space / 1024.0 / 1024.0;
                        let freed_space_str: String = if mb > 1024.0 {
                            format!("{:.2} GB", mb / 1024.0)
                        } else {
                            format!("{:.2} MB", mb)
                        };

                        window
                            .emit(
                                "cleaner-data",
                                json!({
                                    "deleted_files": deleted_files,
                                    "total_files": total_files,
                                    "freed_space": freed_space_str
                                }),
                            )
                            .unwrap();
                    }
                });
        }
    }

    Ok(())
}
