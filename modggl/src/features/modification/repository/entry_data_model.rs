use serde::Deserialize;

use crate::utils;

use super::application::domain::entry::*;

#[derive(Debug, Deserialize, Clone)]
pub struct EntryDataModel {
    pub id: usize,
    pub pid: usize,
    pub uid: usize,
    pub description: String,
    pub start: String,
    pub end: String,
    pub updated: String,
    pub dur: usize,
    pub user: String,
    pub use_stop: bool,
    pub client: String,
    pub project: String,
    pub project_color: String,
    pub project_hex_color: String,
    pub is_billable: bool,
    pub tags: Vec<String>,
    pub tid: Option<usize>,   // 今のプランでは使っていない
    pub task: Option<()>,     // 今のプランでは使っていない
    pub billable: Option<()>, // 今のプランでは使っていない
    pub cur: Option<()>,      // 今のプランでは使っていない
}

impl EntryDataModel {
    pub fn to_domain_model(&self) -> Entry {
        let tags = self.tags.clone();

        Entry::new(EntryProps {
            client: EntryClient::new(self.client.clone()),
            description: EntryDescription::new(self.description.clone()),
            id: EntryId::new(self.id),
            period: EntryPeriod::new(EntryPeriodProps {
                start: EntryStart::new(utils::generate_datetime(self.start.as_str())),
                end: EntryEnd::new(utils::generate_datetime(self.end.as_str())),
            }),
            project: EntryProject::new(self.project.clone()),
            tags: EntryTagList::new(tags.into_iter().map(|e| EntryTag::new(e)).collect()),
            updated_at: EntryUpdated::new(utils::generate_datetime(self.updated.as_str())),
        })
    }
}
