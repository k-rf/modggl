use super::Entry;
use std::slice::Iter;

pub struct EntryList {
    pub value: Vec<Entry>,
}

impl EntryList {
    pub fn as_iter(&self) -> Iter<Entry> {
        self.value.iter()
    }
}

impl Iterator for EntryList {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        (*self).next()
    }
}
