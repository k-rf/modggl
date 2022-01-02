use actix_web::web;

use crate::features::modification::controller::modify;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(modify::handle)));
}
