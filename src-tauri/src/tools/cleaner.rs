use super::KaizenResult;
use serde_json::json;
use std::fs::{self, remove_dir_all};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Runtime};

#[tauri::command]
pub async fn run_cleaner<R: Runtime>(app: AppHandle<R>, dirs: Vec<String>) -> KaizenResult<()> {
    let mut stats: Stats = Stats::default();

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                stats.total += 1;
                if let Ok(metadata) = entry.metadata() {
                    let path: PathBuf = entry.path();
                    let file_size: f64 = metadata.len() as f64;

                    if (if metadata.is_file() {
                        fs::remove_file(&path)
                    } else {
                        remove_dir_all(&path)
                    })
                    .is_ok()
                    {
                        stats.deleted += 1;
                        stats.freed += file_size;
                    }

                    app.emit(
                        "cleaner-data",
                        json!({
                            "deleted": stats.deleted,
                            "total": stats.total,
                            "freed": format_freed(stats.freed)
                        }),
                    )?;
                }
            }
        }
    }

    Ok(())
}

#[derive(Default)]
struct Stats {
    deleted: i32,
    total: i32,
    freed: f64,
}

#[inline]
fn format_freed(bytes: f64) -> String {
    let mb = bytes / 1_048_576.0;
    if mb > 1024.0 {
        format!("{:.2} GB", mb / 1024.0)
    } else {
        format!("{:.2} MB", mb)
    }
}
