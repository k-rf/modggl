use chrono::{DateTime, Utc};

use crate::utils::datetime_generator;

use super::{
    entry_client::EntryClient,
    entry_description::EntryDescription,
    entry_period::{EntryEnd, EntryPeriod, EntryPeriodProps, EntryStart},
    entry_project::EntryProject,
    entry_tag::EntryTag,
    entry_tag_list::EntryTagList,
    entry_updated::EntryUpdated,
    Entry, EntryId,
};

pub struct EntryBuilder {
    client: EntryClient,
    description: EntryDescription,
    end: EntryEnd,
    id: EntryId,
    period: EntryPeriod,
    project: EntryProject,
    start: EntryStart,
    tags: EntryTagList,
    updated_at: EntryUpdated,
}

impl EntryBuilder {
    pub fn new() -> Self {
        let start = EntryStart::new(datetime_generator("1970-01-01T00:00:00+00:00"));
        let end = EntryEnd::new(datetime_generator("1970-01-01T00:00:00+00:00"));

        EntryBuilder {
            client: EntryClient::new(String::from("default client")),
            description: EntryDescription::new(String::from("default description")),
            end,
            id: EntryId::new(9999),
            period: EntryPeriod::new(EntryPeriodProps { start, end }),
            project: EntryProject::new(String::from("default project")),
            start,
            tags: EntryTagList::new(vec![]),
            updated_at: EntryUpdated::new(datetime_generator("2000-01-01T12:00:00+00:00")),
        }
    }

    pub fn client(mut self, value: &str) -> Self {
        self.client = EntryClient::new(String::from(value));
        self
    }

    pub fn description(mut self, value: &str) -> Self {
        self.description = EntryDescription::new(String::from(value));
        self
    }

    pub fn id(mut self, value: usize) -> Self {
        self.id = EntryId::new(value);
        self
    }

    pub fn start(mut self, value: DateTime<Utc>) -> Self {
        self.start = EntryStart::new(value);
        self.updated_at = EntryUpdated::new(value);
        self
    }

    pub fn end(mut self, value: DateTime<Utc>) -> Self {
        self.end = EntryEnd::new(value);
        self
    }

    pub fn project(mut self, value: &str) -> Self {
        self.project = EntryProject::new(String::from(value));
        self
    }

    pub fn tags(mut self, value: Vec<&str>) -> Self {
        self.tags = EntryTagList::new(
            value
                .into_iter()
                .map(|e| EntryTag::new(String::from(e)))
                .collect(),
        );
        self
    }

    pub fn updated_at(mut self, value: DateTime<Utc>) -> Self {
        self.updated_at = EntryUpdated::new(value);
        self
    }

    pub fn build(self) -> Entry {
        Entry {
            client: self.client,
            description: self.description,
            id: self.id,
            period: EntryPeriod::new(EntryPeriodProps {
                start: self.start,
                end: self.end,
            }),
            project: self.project,
            tags: self.tags,
            updated_at: self.updated_at,
        }
    }
}
