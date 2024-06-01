use chrono::{DateTime, Utc};

pub struct ModifyEntryCommand {
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
}
