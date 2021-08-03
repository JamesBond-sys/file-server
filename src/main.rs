use actix_web::{App, HttpServer, middleware::Logger};
use actix_files::{Files};
use crate::routes::routes::app_config;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";

    println!("Starting server at http://{}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                Files::new("/", ".")
                    .show_files_listing()
                    .use_last_modified(true)
            )
            .configure(app_config)
    })
        .bind(address)?
        .run()
        .await
}