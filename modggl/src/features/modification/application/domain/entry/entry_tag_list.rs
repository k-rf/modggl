use super::entry_tag::EntryTag;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntryTagList {
    pub value: Vec<EntryTag>,
}

impl EntryTagList {
    pub fn new(value: Vec<EntryTag>) -> Self {
        EntryTagList { value }
    }
}
