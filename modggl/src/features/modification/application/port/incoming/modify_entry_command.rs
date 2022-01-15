use chrono::{Date, Utc};

pub struct ModifyEntryCommand {
    pub since: Date<Utc>,
    pub until: Date<Utc>,
}
