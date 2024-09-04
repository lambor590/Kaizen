use super::KaizenResult;
use lazy_static::lazy_static;
use reqwest::get;
use serde::Deserialize;
use std::io::Cursor;
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Output},
};
use zip::ZipArchive;

#[derive(Deserialize)]
pub struct DownloaderConfig {
    video_url: String,
    format: String,
    quality: String,
    output_folder: String,
}

lazy_static! {
    static ref YT_DLP_PATH: PathBuf = {
        let mut path = env::current_exe().expect("Failed to get current executable path");
        path.pop();
        path.push("yt-dlp");
        path
    };
}

#[tauri::command]
pub async fn run_downloader(config: DownloaderConfig) -> KaizenResult<()> {
    let (output_format, extra_args): (&str, Vec<&str>) =
        match (config.format.as_str(), config.quality.as_str()) {
            ("video", "best") => ("--format", vec!["bestvideo/b", "--remux-video", "mp4"]),
            ("audio", "best") => ("-x", vec!["--audio-format", "mp3", "--audio-quality", "0"]),
            ("both", "best") => (
                "--format",
                vec!["bestvideo+bestaudio/b", "--remux-video", "mp4"],
            ),
            (_, other) => (other, vec![]),
        };

    let mut args: Vec<&str> = vec![
        "--no-playlist",
        "--quiet",
        "-P",
        config.output_folder.as_str(),
        "-P",
        "temp:%temp%",
        output_format,
    ];

    args.extend_from_slice(&extra_args);
    args.push(&config.video_url);

    Command::new(&*YT_DLP_PATH).args(&args).output().unwrap();

    Ok(())
}

#[tauri::command]
pub async fn get_video_data(url: String) -> Result<String, String> {
    let output: Output = Command::new(&*YT_DLP_PATH)
        .args(&["-J", "--no-warnings", &url])
        .output()
        .map_err(|e: std::io::Error| e.to_string())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e: std::string::FromUtf8Error| e.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
pub async fn check_downloader_deps() -> bool {
    Command::new(&*YT_DLP_PATH)
        .arg("--version")
        .output()
        .map(|output: Output| output.status.success())
        .unwrap_or(false)
}

#[tauri::command]
pub async fn install_downloader_deps() -> KaizenResult<()> {
    let exe_dir: PathBuf = env::current_exe()?
        .parent()
        .ok_or("Failed to get exe directory")
        .unwrap()
        .to_path_buf();

    async fn download_and_save(
        url: &str,
        path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(path, get(url).await?.bytes().await?)?;
        Ok(())
    }

    download_and_save(
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
        &exe_dir.join("yt-dlp.exe"),
    )
    .await
    .unwrap();

    let ffmpeg_response = get("https://github.com/yt-dlp/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip").await.unwrap().bytes().await.unwrap();
    let mut ffmpeg_zip = ZipArchive::new(Cursor::new(ffmpeg_response)).unwrap();

    for i in 0..ffmpeg_zip.len() {
        let mut file: zip::read::ZipFile<'_> = ffmpeg_zip.by_index(i).unwrap();
        if let Some(path) = file
            .enclosed_name()
            .filter(|p: &PathBuf| p.starts_with("ffmpeg-master-latest-win64-gpl/bin"))
        {
            let outpath: PathBuf =
                exe_dir.join(path.file_name().ok_or("Failed to get file name").unwrap());
            let mut outfile: fs::File = fs::File::create(outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
