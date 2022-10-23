use actix_web::{get, web, HttpResponse, Responder};

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("1.0.0")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(version);
}
