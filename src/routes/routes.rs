use actix_web::{web, HttpResponse, web::ServiceConfig};

pub fn app_config(config: &mut ServiceConfig) {
    let beat = web::resource("/beat")
        .route(web::get().to(heart_beat));

    config.service(beat);
}

async fn heart_beat() -> HttpResponse {
    HttpResponse::Ok().finish()
}