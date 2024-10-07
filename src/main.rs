use actix_web::{web, App, HttpServer};

mod models;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/venues/{id}", web::get().to(handlers::venue::get_venue))
            .route("/venues", web::post().to(handlers::venue::create_venue))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}