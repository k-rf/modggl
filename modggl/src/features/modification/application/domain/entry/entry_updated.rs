use chrono::{DateTime, Utc};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryUpdated {
    pub value: DateTime<Utc>,
}

impl EntryUpdated {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryUpdated { value }
    }
}
