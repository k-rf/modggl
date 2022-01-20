use async_trait::async_trait;
use reqwest;
use serde::Serialize;
use url::Url;

use crate::features::modification::application::port::outgoing::EntryModifiedPresenterPort;
use crate::services::env_service;

use super::application::port::outgoing::EntryModifiedOutputData;

#[derive(Debug, Serialize)]
struct PostRequest {
    text: String,
}

pub struct EntryModifiedSlackPresenter {
    pub client: reqwest::Client,
    pub webhook_url: String,
}

impl EntryModifiedSlackPresenter {
    pub fn new() -> Self {
        EntryModifiedSlackPresenter {
            client: reqwest::Client::new(),
            webhook_url: env_service::slack_web_hook_url(),
        }
    }
}

#[async_trait]
impl EntryModifiedPresenterPort for EntryModifiedSlackPresenter {
    async fn execute(&self, value: EntryModifiedOutputData) {
        let data = PostRequest {
            text: value.message,
        };

        self.client
            .post(Url::parse(self.webhook_url.as_str()).unwrap())
            .json(&data)
            .send()
            .await
            .unwrap();
    }
}
