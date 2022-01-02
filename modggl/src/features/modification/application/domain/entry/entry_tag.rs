#[derive(PartialEq, Eq)]
pub struct EntryTag {
    pub value: String,
}

impl EntryTag {
    pub fn new(value: String) -> Self {
        EntryTag { value }
    }
}
