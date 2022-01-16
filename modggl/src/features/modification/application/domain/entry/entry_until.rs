use chrono::{Date, Utc};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryUntil {
    pub value: Date<Utc>,
}

impl EntryUntil {
    pub fn new(value: Date<Utc>) -> Self {
        EntryUntil { value }
    }
}
