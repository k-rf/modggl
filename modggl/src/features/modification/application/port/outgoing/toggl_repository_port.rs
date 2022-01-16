use async_trait::async_trait;

use super::domain::entry::{Entry, EntryList, EntrySince, EntryUntil};

#[async_trait]
pub trait EntryTogglRepositoryPort {
    async fn get(&self, since: EntrySince, until: EntryUntil) -> EntryList;
    async fn modify(&self, value: Entry);
    async fn delete(&self, value: Entry);
}
