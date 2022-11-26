use serde::Serialize;

use super::application::domain::entry::Entry;

#[derive(Debug, Serialize, Clone)]
pub struct EntryRequestModel {
    start: String,
    stop: String,
    duration: i64,
}

impl EntryRequestModel {
    pub fn from(value: &Entry) -> Self {
        EntryRequestModel {
            start: value.period.start.value.to_rfc3339(),
            stop: value.period.end.value.to_rfc3339(),
            duration: value.period.duration.value,
        }
    }
}
