use chrono::{DateTime, Utc};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryUntil {
    pub value: DateTime<Utc>,
}

impl EntryUntil {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryUntil { value }
    }
}
