#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntryId {
    pub value: String,
}

impl EntryId {
    pub fn new(value: String) -> EntryId {
        EntryId { value }
    }
}
