use chrono::{DateTime, Utc};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntrySince {
    pub value: DateTime<Utc>,
}

impl EntrySince {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntrySince { value }
    }
}
