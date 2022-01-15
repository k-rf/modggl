use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::services::env_service;
use crate::utils;

use super::application::domain::entry::{EntryBuilder, EntryList, EntrySince, EntryUntil};
use super::application::port::outgoing::EntryTogglRepositoryPort;
use super::EntryDataModel;

#[derive(Debug, Deserialize)]
pub struct GetResponse {
    pub data: Vec<EntryDataModel>,
}

pub struct EntryTogglRepository {
    client: reqwest::Client,
    workspace_id: String,
    user_agent: String,
    token: String,
    token_type: String,
}

impl EntryTogglRepository {
    pub fn new() -> Self {
        EntryTogglRepository {
            client: reqwest::Client::new(),
            workspace_id: env_service::workspace_id(),
            user_agent: env_service::user_agent(),
            token: env_service::token(),
            token_type: env_service::token_type(),
        }
    }
}

#[async_trait]
impl EntryTogglRepositoryPort for EntryTogglRepository {
    async fn get(&self, since: EntrySince, until: EntryUntil) -> EntryList {
        let base_url = format!("{}/{}", env_service::report_api(), "details");
        let divided = utils::date_divider(since.value, until.value);

        let username = self.token.as_str();
        let password = Some(self.token_type.as_str()); // Toggl API の仕様

        let mut entry_list = EntryList::new(vec![]);
        let mut i = 1;

        for (since, until) in divided {
            loop {
                let page = i.to_string();

                let response = self
                    .client
                    .get(
                        Url::parse_with_params(
                            base_url.as_str(),
                            vec![
                                ("workspace_id", self.workspace_id.clone()),
                                ("user_agent", self.user_agent.clone()),
                                ("order_field", String::from("date")),
                                ("order_desc", String::from("off")),
                                ("since", since.to_string().clone()),
                                ("until", until.to_string().clone()),
                                ("page", page),
                            ],
                        )
                        .unwrap(),
                    )
                    .basic_auth(username, password)
                    .send()
                    .await
                    .unwrap();

                let data_model = response.json::<GetResponse>().await.unwrap().data;

                if data_model.len() == 0 {
                    break;
                }

                for entry in data_model.into_iter() {
                    entry_list.insert(entry.to_domain_model());
                }

                i += 1;
            }
        }

        entry_list
    }

    async fn modify(&self) {}
    async fn delete(&self) {}
}

#[cfg(test)]
mod tests {
    use tokio;

    use crate::utils::date_generator;

    use super::*;

    #[tokio::test]
    async fn test_toggl_repository() {
        dotenv::from_filename(".env.local").ok();

        let repository = EntryTogglRepository::new();
        let res = repository
            .get(
                EntrySince::new(date_generator("2022-01-15")),
                EntryUntil::new(date_generator("2022-01-31")),
            )
            .await;
    }
}
