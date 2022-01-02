#[derive(PartialEq, Eq)]
pub struct EntryDescription {
    pub value: String,
}

impl EntryDescription {
    pub fn new(value: String) -> Self {
        EntryDescription { value }
    }
}
