mod config;
mod recorder;

use chrono::{Local, Timelike};
use sqlx::postgres::PgPoolOptions;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;

    let db_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("Connected to the database successfully!");

    let streams = sqlx::query!("SELECT name, rtsp_url FROM camera")
        .fetch_all(&pool)
        .await?;

    let mut healthy_streams = Vec::new();
    for stream in &streams {
        println!("Checking camera: {}", stream.name);
        if recorder::check_rtsp_health(&stream.rtsp_url).await {
            println!("Camera {} health: OK", stream.name);
            healthy_streams.push((stream.name.clone(), stream.rtsp_url.clone()));
        } else {
            println!("Camera {} health: DISCONNECTED", stream.name);
        }
    }

    loop {
        let now = Local::now();
        let seconds_until_next_minute = 60 - now.second();
        sleep(Duration::from_secs(seconds_until_next_minute as u64)).await;

        for (camera_name, rtsp_url) in &healthy_streams {
            let camera_name = camera_name.clone();
            let rtsp_url = rtsp_url.clone();
            let records_path = config.app.records_path.clone();
            tokio::spawn(async move {
                recorder::record_minute(&records_path, &camera_name, &rtsp_url, 60).await;
            });
        }
    }
}