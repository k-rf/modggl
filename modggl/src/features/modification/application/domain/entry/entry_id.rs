#[derive(Debug)]
pub struct EntryId {
    pub value: String,
}

impl EntryId {
    pub fn new() -> EntryId {
        EntryId {
            value: String::from("abc"),
        }
    }
}
