use async_trait::async_trait;

use crate::utils;

use super::application::domain::entry::{Entry, EntryBuilder, EntryList, EntrySince, EntryUntil};
use super::application::port::outgoing::EntryTogglRepositoryPort;

pub struct MockTogglRepository {}

#[async_trait]
impl EntryTogglRepositoryPort for MockTogglRepository {
    async fn get(&self, _since: EntrySince, _until: EntryUntil) -> EntryList {
        EntryList {
            value: vec![
                EntryBuilder::new()
                    .id(1)
                    .client("a")
                    .start(utils::datetime_generator("2000-01-01T00:00:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:10:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(2)
                    .client("b")
                    .start(utils::datetime_generator("2000-01-01T00:11:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:25:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(22)
                    .client("b")
                    .start(utils::datetime_generator("2000-01-01T00:25:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:30:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(222)
                    .client("b")
                    .start(utils::datetime_generator("2000-01-01T00:30:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T00:35:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(3)
                    .client("c")
                    .start(utils::datetime_generator("2000-01-01T00:32:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T01:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(4)
                    .client("d")
                    .start(utils::datetime_generator("2000-01-01T01:00:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T01:43:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(5)
                    .client("e")
                    .start(utils::datetime_generator("2000-01-01T01:10:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T01:30:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(6)
                    .client("f")
                    .start(utils::datetime_generator("2000-01-01T01:43:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T03:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .id(6)
                    .client("f")
                    .start(utils::datetime_generator("2000-01-01T01:43:00+00:00"))
                    .end(utils::datetime_generator("2000-01-01T03:00:00+00:00"))
                    .build(),
            ],
        }
    }
    async fn modify(&self, _value: Entry) {}
    async fn delete(&self, _value: Entry) {}
}
