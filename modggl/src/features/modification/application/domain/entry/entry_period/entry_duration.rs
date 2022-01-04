#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EntryDuration {
    pub value: i64,
}

impl EntryDuration {
    pub fn new(value: i64) -> Self {
        EntryDuration { value }
    }
}
