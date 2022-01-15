#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntryId {
    pub value: usize,
}

impl EntryId {
    pub fn new(value: usize) -> EntryId {
        EntryId { value }
    }
}
