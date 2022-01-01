pub struct EntryDescription {
    pub value: String,
}

impl EntryDescription {
    pub fn new(value: String) -> Self {
        EntryDescription { value }
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
