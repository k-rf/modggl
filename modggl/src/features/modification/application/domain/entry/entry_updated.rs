use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EntryUpdated {
    pub value: DateTime<Utc>,
}

impl EntryUpdated {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryUpdated { value }
    }
}
