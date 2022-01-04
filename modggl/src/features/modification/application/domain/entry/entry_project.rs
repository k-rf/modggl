#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntryProject {
    pub value: String,
}

impl EntryProject {
    pub fn new(value: String) -> Self {
        EntryProject { value }
    }
}
