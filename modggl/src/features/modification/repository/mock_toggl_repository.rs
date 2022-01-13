use async_trait::async_trait;

use crate::utils;

use super::application::domain::entry::{EntryBuilder, EntryList, EntrySince, EntryUntil};
use super::application::port::outgoing::TogglRepositoryPort;

pub struct MockTogglRepository {}

#[async_trait]
impl TogglRepositoryPort for MockTogglRepository {
    async fn get(&self, since: EntrySince, until: EntryUntil) -> EntryList {
        // Todo
        EntryList {
            value: vec![
                EntryBuilder::new()
                    .id("a")
                    .client("a")
                    .start(utils::datetime_generator("2000-01-01T00:00:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:10:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("b")
                    .client("b")
                    .start(utils::datetime_generator("2000-01-01T00:11:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:25:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("bb")
                    .client("b")
                    .start(utils::datetime_generator("2000-01-01T00:25:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:30:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("bbb")
                    .client("b")
                    .start(utils::datetime_generator("2000-01-01T00:30:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:35:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("c")
                    .client("c")
                    .start(utils::datetime_generator("2000-01-01T00:32:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T01:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("d")
                    .client("d")
                    .start(utils::datetime_generator("2000-01-01T01:00:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T01:43:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("e")
                    .client("e")
                    .start(utils::datetime_generator("2000-01-01T01:10:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T01:30:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("f")
                    .client("f")
                    .start(utils::datetime_generator("2000-01-01T01:43:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T03:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id("f")
                    .client("f")
                    .start(utils::datetime_generator("2000-01-01T01:43:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T03:00:00+00:00"))
                    .build(),
            ],
        }
    }
    async fn modify(&self) {}
    async fn delete(&self) {}
}
