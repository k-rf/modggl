#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntryDescription {
    pub value: String,
}

impl EntryDescription {
    pub fn new(value: String) -> Self {
        EntryDescription { value }
    }
}
