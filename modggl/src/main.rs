use actix_web::{App, HttpServer};
use dotenv;

use modggl::routes::config::config;
use modggl::services::LoggerService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LoggerService::init();
    dotenv::from_filename(".env.local").ok();

    HttpServer::new(|| App::new().configure(config))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
