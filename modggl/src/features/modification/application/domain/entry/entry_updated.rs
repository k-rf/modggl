use chrono::{DateTime, Utc};

pub struct EntryUpdated {
    pub value: DateTime<Utc>,
}

impl EntryUpdated {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryUpdated { value }
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
