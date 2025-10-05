use actix_web::{web, HttpResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/streams", web::get().to(get_streams));
}

async fn get_streams() -> HttpResponse {
    // TODO: Return list of streams from database
    HttpResponse::Ok().body("Streams list")
}