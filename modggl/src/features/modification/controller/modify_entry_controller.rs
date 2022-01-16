use std::sync::Arc;

use actix_web::Responder;
use chrono::{Duration, Utc};

use crate::features::modification::application::port::incoming::ModifyEntryCommand;

use super::application::port::incoming::ModifyEntryUsecase;

pub struct ModifyEntryController {
    pub usecase: Arc<dyn ModifyEntryUsecase + Sync + Send>,
}

impl ModifyEntryController {
    pub async fn handle(&self) -> impl Responder {
        let week = Duration::days(7);
        let day = Duration::days(1);

        let command = ModifyEntryCommand {
            since: Utc::today() - week,
            until: Utc::today() + day,
        };

        self.usecase.execute(command).await;

        format!("Modification is completed.")
    }
}
