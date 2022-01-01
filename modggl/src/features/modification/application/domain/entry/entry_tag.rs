pub struct EntryTag {
    pub value: String,
}

impl EntryTag {
    pub fn new(value: String) -> Self {
        EntryTag { value }
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
