use super::KaizenResult;
use rayon::prelude::*;
use serde_json::json;
use std::fs::{self, remove_dir_all};
use tauri::{AppHandle, Emitter, Runtime};

#[tauri::command]
pub async fn run_cleaner<R: Runtime>(app: AppHandle<R>, dirs: Vec<String>) -> KaizenResult<()> {
    let stats: Stats = dirs
        .par_iter()
        .map(|dir: &String| match fs::read_dir(dir) {
            Ok(entries) => entries
                .filter_map(Result::ok)
                .par_bridge()
                .map(|entry: fs::DirEntry| {
                    let mut stats: Stats = Stats::default();
                    if let Ok(metadata) = entry.metadata() {
                        stats.total = 1;
                        if (if metadata.is_file() {
                            fs::remove_file(entry.path())
                        } else {
                            remove_dir_all(entry.path())
                        })
                        .is_ok()
                        {
                            stats.deleted = 1;
                            stats.freed = metadata.len() as f64;
                        }
                    }
                    stats
                })
                .reduce(Stats::default, |a: Stats, b: Stats| a.merge(&b)),
            Err(_) => Stats::default(),
        })
        .reduce(Stats::default, |a: Stats, b: Stats| a.merge(&b));

    app.emit(
        "cleaner-data",
        json!({
            "deleted": stats.deleted,
            "total": stats.total,
            "freed": format_freed(stats.freed)
        }),
    )?;

    Ok(())
}

#[derive(Default, Clone, Copy)]
pub struct Stats {
    deleted: i32,
    total: i32,
    freed: f64,
}

impl Stats {
    #[inline(always)]
    fn merge(&self, other: &Stats) -> Self {
        Stats {
            deleted: self.deleted + other.deleted,
            total: self.total + other.total,
            freed: self.freed + other.freed,
        }
    }
}

#[inline(always)]
fn format_freed(bytes: f64) -> String {
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MB: f64 = 1024.0 * 1024.0;

    if bytes >= GB {
        format!("{:.2} GB", bytes / GB)
    } else {
        format!("{:.2} MB", bytes / MB)
    }
}
