use actix_web::{App, HttpServer};
mod rtsp_handler;
mod hls_recorder;
mod minio_client;
mod db;
mod api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api::init_routes))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}