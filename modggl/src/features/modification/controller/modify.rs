use actix_web::Responder;

// use super::application::modification_usecase;

pub async fn handle() -> impl Responder {
    // let result = modification_usecase::execute();
    format!("{}", 12)
}
