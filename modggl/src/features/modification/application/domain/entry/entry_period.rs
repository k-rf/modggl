use super::{entry_end::EntryEnd, entry_start::EntryStart};

pub struct EntryPeriod {
    start: EntryStart,
    end: EntryEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_implemented() {
        panic!("Fail!")
    }
}
