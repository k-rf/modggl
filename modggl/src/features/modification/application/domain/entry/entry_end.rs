use chrono::{DateTime, Utc};

pub struct EntryEnd {
    pub value: DateTime<Utc>,
}

impl EntryEnd {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryEnd { value }
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
