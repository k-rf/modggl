use std::sync::Arc;

use actix_web::{web, Responder};

use crate::features::modification::application::usecase::ModifyEntryInteractor;
use crate::features::modification::controller::ModifyEntryController;
use crate::features::modification::repository::EntryTogglRepository;

async fn modify() -> impl Responder {
    let usecase = ModifyEntryInteractor {
        toggl_repository_port: Arc::new(EntryTogglRepository::new()),
    };
    let controller = ModifyEntryController {
        usecase: Arc::new(usecase),
    };

    controller.handle().await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(modify)));
}
