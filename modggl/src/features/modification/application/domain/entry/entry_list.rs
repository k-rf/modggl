use super::Entry;

pub struct EntryList {
    pub value: Vec<Entry>,
}

impl EntryList {
    pub fn new(value: Vec<Entry>) -> Self {
        EntryList { value }
    }

    pub fn insert(&mut self, other: Entry) {
        self.value.insert(self.value.len(), other);
    }

    pub fn upsert(&mut self, other: Entry) {
        match self.value.iter().position(|e| e.id == other.id) {
            Some(i) => {
                self.value.remove(i);
                self.value.insert(i, other);
            }
            None => {
                self.insert(other);
            }
        }
    }
}
