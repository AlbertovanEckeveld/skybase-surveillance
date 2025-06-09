use chrono::{Datelike, Local, Timelike};
use std::fs;
use std::process::Stdio;
use tokio::process::Command;

pub async fn check_rtsp_health(rtsp_url: &str) -> bool {
    let output = Command::new("ffmpeg")
        .args([
            "-rtsp_transport", "tcp",
            "-i", rtsp_url,
            "-t", "1",
            "-f", "null",
            "-",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .await;

    matches!(output, Ok(result) if result.status.success())
}

pub async fn record_minute(records_path: &str, camera_name: &str, rtsp_url: &str, duration: u32) {
    let now = Local::now();
    let path = format!(
        "{}/{}/{}/{:02}/{:02}/{:02}/{:02}",
        records_path,
        camera_name,
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute()
    );

    if let Err(e) = fs::create_dir_all(&path) {
        eprintln!("Failed to create directory {}: {}", path, e);
        return;
    }

    let output_file = format!("{}/record.mp4", path);

    let ffmpeg_args = [
        "-rtsp_transport", "tcp",
        "-i", rtsp_url,
        "-t", &duration.to_string(),
        "-an",
        "-c:v", "libx264",
        "-preset", "veryfast",
        "-crf", "23",
        &output_file,
        "-y",
    ];

    match Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(mut child) => {
            if let Err(e) = child.wait().await {
                eprintln!("FFmpeg process failed: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to spawn ffmpeg for camera {}: {}", camera_name, e);
        }
    }
}