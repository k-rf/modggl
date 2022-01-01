pub struct EntryDuration {
    pub value: i128,
}

impl EntryDuration {
    pub fn new(value: i128) -> Self {
        EntryDuration { value }
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
