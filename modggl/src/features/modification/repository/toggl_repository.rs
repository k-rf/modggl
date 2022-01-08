use async_trait::async_trait;

use super::application::domain::entry::{EntryBuilder, EntryList, EntrySince, EntryUntil};
use super::application::port::outgoing::TogglRepositoryPort;

pub struct TogglRepository {}

#[async_trait]
impl TogglRepositoryPort for TogglRepository {
    async fn get(&self, since: EntrySince, until: EntryUntil) -> EntryList {
        // Todo
        EntryList {
            value: vec![
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
            ],
        }
    }
    async fn modify(&self) {}
    async fn delete(&self) {}
}
