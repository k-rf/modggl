#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntryClient {
    pub value: String,
}

impl EntryClient {
    pub fn new(value: String) -> Self {
        EntryClient { value }
    }
}
