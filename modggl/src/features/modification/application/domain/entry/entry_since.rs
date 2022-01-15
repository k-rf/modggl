use chrono::{Date, Utc};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntrySince {
    pub value: Date<Utc>,
}

impl EntrySince {
    pub fn new(value: Date<Utc>) -> Self {
        EntrySince { value }
    }

    pub fn to_string(&self) -> String {
        self.value.format("%Y-%m-%d").to_string()
    }
}
