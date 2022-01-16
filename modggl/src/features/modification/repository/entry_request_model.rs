use serde::Serialize;

use super::application::domain::entry::Entry;

#[derive(Debug, Serialize, Clone)]
pub struct EntryRequestModel {
    start: String,
    end: String,
    duration: i64,
    updated: String,
}

impl EntryRequestModel {
    pub fn from(value: &Entry) -> Self {
        EntryRequestModel {
            start: value.period.start.value.to_rfc3339(),
            end: value.period.end.value.to_rfc3339(),
            duration: value.period.duration.value,
            updated: value.updated_at.value.to_rfc3339(),
        }
    }
}
