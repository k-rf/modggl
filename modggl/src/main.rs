use actix_web::{App, HttpServer};

use modggl::routes::config::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
