use chrono::{DateTime, Utc};

pub struct EntryStart {
    pub value: DateTime<Utc>,
}

impl EntryStart {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryStart { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_implemented() {
        panic!("Fail!")
    }
}
