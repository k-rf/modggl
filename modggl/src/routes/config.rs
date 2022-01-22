use std::sync::Arc;

use actix_web::{web, Responder};

use crate::features::modification::application::usecase::ModifyEntryInteractor;
use crate::features::modification::controller::ModifyEntryController;
use crate::features::modification::presenter::EntryModifiedSlackPresenter;
use crate::features::modification::repository::EntryTogglRepository;

async fn modify() -> impl Responder {
    let usecase = ModifyEntryInteractor {
        entry_repository_port: Arc::new(EntryTogglRepository::new()),
        entry_modified_presenter_port: Arc::new(EntryModifiedSlackPresenter::new()),
    };
    let controller = ModifyEntryController {
        usecase: Arc::new(usecase),
    };

    controller.handle().await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::post().to(modify)));
}
