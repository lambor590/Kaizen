use super::KaizenResult;
use lazy_static::lazy_static;
use reqwest::get;
use serde::Deserialize;
use std::io::Cursor;
use std::os::windows::process::CommandExt;
use std::{env, fs, path::PathBuf, process::Command};
use winapi::um::winbase::CREATE_NO_WINDOW;
use zip::ZipArchive;

#[derive(Deserialize)]
pub struct DownloaderConfig {
    video_url: String,
    format: String,
    quality: String,
    pitch: String,
    output_folder: String,
}

lazy_static! {
    static ref YT_DLP_PATH: PathBuf = env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .unwrap()
        .join("yt-dlp");
}

#[tauri::command]
pub async fn run_downloader(config: DownloaderConfig) -> KaizenResult<()> {
    let extra_args: Vec<&str> = match (config.format.as_str(), config.quality.as_str()) {
        ("video", "best") => vec!["--format", "bestvideo/b", "--remux-video", "mp4"],
        ("both", "best") => vec!["--format", "bestvideo+bestaudio/b", "--remux-video", "mp4"],
        ("audio", _) => vec!["-x", "--audio-format", "mp3", "--audio-quality", "0"],
        (_, other) => vec![other],
    };

    let temp_dir: PathBuf = env::temp_dir();
    let output_dir: PathBuf = PathBuf::from(&config.output_folder);

    let mut args: Vec<String> = vec![
        "--no-playlist".to_string(),
        "--quiet".to_string(),
        "-P".to_string(),
    ];

    let initial_output_dir: &str = if config.pitch != "1.0" {
        temp_dir.to_str().unwrap()
    } else {
        output_dir.to_str().unwrap()
    };

    args.push(initial_output_dir.to_string());
    args.extend(extra_args.iter().map(|s: &&str| s.to_string()));
    args.push(config.video_url.clone());

    Command::new(&*YT_DLP_PATH)
        .args(&args)
        .creation_flags(CREATE_NO_WINDOW)
        .status()?;

    if config.pitch == "1.0" {
        return Ok(());
    };

    let downloaded_file: fs::DirEntry = fs::read_dir(temp_dir)?
        .filter_map(Result::ok)
        .filter(|entry: &fs::DirEntry| {
            entry
                .file_name()
                .to_string_lossy()
                .ends_with(if config.format == "audio" {
                    ".mp3"
                } else {
                    ".mp4"
                })
        })
        .last()
        .ok_or("No se encontró el archivo descargado")
        .unwrap();

    let input_path: PathBuf = downloaded_file.path();
    let output_path: PathBuf = output_dir.join(downloaded_file.file_name());

    Command::new("ffmpeg")
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-filter:a",
            &format!("asetrate=44100*{},aresample=44100", config.pitch),
            "-y",
            output_path.to_str().unwrap(),
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

    fs::remove_file(input_path)?;

    Ok(())
}

#[tauri::command]
pub async fn get_video_data(url: String) -> Result<String, String> {
    let output: std::process::Output = Command::new(&*YT_DLP_PATH)
        .args(["-J", "--no-warnings", &url])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e: std::io::Error| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e: std::string::FromUtf8Error| e.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
    .map_err(|e: String| e.to_string())
}

#[tauri::command]
pub async fn check_downloader_deps() -> bool {
    Command::new(&*YT_DLP_PATH)
        .arg("--version")
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map_or(false, |status: std::process::ExitStatus| status.success())
}

#[tauri::command]
pub async fn install_downloader_deps() -> KaizenResult<()> {
    let exe_dir: PathBuf = env::current_exe()?
        .parent()
        .ok_or("Failed to get exe directory")
        .unwrap()
        .to_path_buf();

    async fn download_and_save(url: &str, path: &PathBuf) -> KaizenResult<()> {
        let bytes = get(url).await.unwrap().bytes().await.unwrap();
        fs::write(path, bytes)?;
        Ok(())
    }

    download_and_save(
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
        &exe_dir.join("yt-dlp.exe"),
    )
    .await?;

    let ffmpeg_bytes = get("https://github.com/yt-dlp/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let mut ffmpeg_zip = ZipArchive::new(Cursor::new(ffmpeg_bytes)).unwrap();

    for i in 0..ffmpeg_zip.len() {
        let mut file: zip::read::ZipFile<'_> = ffmpeg_zip.by_index(i).unwrap();
        if let Some(name) = file.enclosed_name() {
            if name.starts_with("ffmpeg-master-latest-win64-gpl/bin") {
                let outpath: PathBuf = exe_dir.join(name.file_name().unwrap());
                std::io::copy(&mut file, &mut fs::File::create(outpath)?)?;
            }
        }
    }

    Ok(())
}
