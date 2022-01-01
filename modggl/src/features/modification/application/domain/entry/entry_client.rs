pub struct EntryClient {
    pub value: String,
}

impl EntryClient {
    pub fn new(value: String) -> Self {
        EntryClient { value }
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
