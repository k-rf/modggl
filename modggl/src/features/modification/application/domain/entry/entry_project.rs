pub struct EntryProject {
    pub value: String,
}

impl EntryProject {
    pub fn new(value: String) -> Self {
        EntryProject { value }
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
