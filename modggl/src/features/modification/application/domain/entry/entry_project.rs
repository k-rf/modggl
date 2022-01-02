#[derive(PartialEq, Eq)]
pub struct EntryProject {
    pub value: String,
}

impl EntryProject {
    pub fn new(value: String) -> Self {
        EntryProject { value }
    }
}
