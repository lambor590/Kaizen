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
    let (output_format, extra_args): (&str, Vec<&str>) =
        match (config.format.as_str(), config.quality.as_str()) {
            ("video", "best") => ("--format", vec!["bestvideo/b", "--remux-video", "mp4"]),
            ("both", "best") => (
                "--format",
                vec!["bestvideo+bestaudio/b", "--remux-video", "mp4"],
            ),
            ("audio", _) => ("-x", vec!["--audio-format", "mp3", "--audio-quality", "0"]),
            (_, other) => (other, vec![]),
        };

    let mut args: Vec<&str> = vec![
        "--no-playlist",
        "--quiet",
        "-P",
        &config.output_folder,
        "-P",
        "temp:%temp%",
        output_format,
    ];
    args.extend(extra_args);
    args.push(&config.video_url);

    // pitch testing
    // "--exec",
    // "ffmpeg -i {} -filter:a asetrate=44100*1.4,aresample=44100,atempo=1/1.286 \"test.mp3\"",
    // let output_file: String = format!("{}/yt-dlp_output.log", config.output_folder);
    // let file: fs::File = fs::File::create(&output_file)?;
    // .stdout(std::process::Stdio::from(file.try_clone()?))
    // .stderr(std::process::Stdio::from(file))

    Command::new(&*YT_DLP_PATH)
        .args(&args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

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
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

#[tauri::command]
pub async fn check_downloader_deps() -> bool {
    Command::new(&*YT_DLP_PATH)
        .arg("--version")
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map(|status: std::process::ExitStatus| status.success())
        .unwrap_or(false)
}

#[tauri::command]
pub async fn install_downloader_deps() -> KaizenResult<()> {
    let exe_dir: PathBuf = env::current_exe()?
        .parent()
        .ok_or("Failed to get exe directory")
        .unwrap()
        .to_path_buf();

    async fn download_and_save(url: &str, path: &PathBuf) -> KaizenResult<()> {
        fs::write(path, get(url).await.unwrap().bytes().await.unwrap()).unwrap();
        Ok(())
    }

    download_and_save(
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
        &exe_dir.join("yt-dlp.exe"),
    )
    .await?;

    let ffmpeg_response = get("https://github.com/yt-dlp/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip").await.unwrap().bytes().await.unwrap();
    let mut ffmpeg_zip = ZipArchive::new(Cursor::new(ffmpeg_response)).unwrap();

    for i in 0..ffmpeg_zip.len() {
        let mut file: zip::read::ZipFile<'_> = ffmpeg_zip.by_index(i).unwrap();
        if let Some(name) = file.enclosed_name() {
            if name.starts_with("ffmpeg-master-latest-win64-gpl/bin") {
                let outpath: PathBuf = exe_dir.join(name.file_name().unwrap());
                let mut outfile: fs::File = fs::File::create(outpath)?;
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }

    Ok(())
}
