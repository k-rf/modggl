use std::cmp;

use chrono::{DateTime, Utc};

use super::EntryStart;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct EntryEnd {
    pub value: DateTime<Utc>,
}

impl EntryEnd {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryEnd { value }
    }
}

impl cmp::PartialEq<EntryStart> for EntryEnd {
    fn eq(&self, other: &EntryStart) -> bool {
        self.value == other.value
    }
}

impl cmp::PartialOrd<EntryStart> for EntryEnd {
    fn partial_cmp(&self, other: &EntryStart) -> Option<cmp::Ordering> {
        if self.lt(other) {
            Some(cmp::Ordering::Less)
        } else if self.gt(other) {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Equal)
        }
    }

    fn le(&self, other: &EntryStart) -> bool {
        self.value <= other.value
    }

    fn lt(&self, other: &EntryStart) -> bool {
        self.value < other.value
    }

    fn ge(&self, other: &EntryStart) -> bool {
        self.value >= other.value
    }

    fn gt(&self, other: &EntryStart) -> bool {
        self.value > other.value
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::utils;

    use super::*;

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("2021-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2021-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2021-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2022-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_equal(this: EntryEnd, that: EntryStart, expected: bool) {
        assert_eq!(this == that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_less_than(this: EntryEnd, that: EntryStart, expected: bool) {
        assert_eq!(this < that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2100-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_less_than_or_equal_to(this: EntryEnd, that: EntryStart, expected: bool) {
        assert_eq!(this <= that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_greater_than(this: EntryEnd, that: EntryStart, expected: bool) {
        assert_eq!(this > that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("2100-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_greater_than_or_equal_to(this: EntryEnd, that: EntryStart, expected: bool) {
        assert_eq!(this >= that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            cmp::Ordering::Equal
        ),
        case(
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            cmp::Ordering::Less
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryStart::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            cmp::Ordering::Greater
        )
    )]
    fn test_compare(this: EntryEnd, that: EntryStart, expected: cmp::Ordering) {
        assert_eq!(this.partial_cmp(&that).unwrap(), expected)
    }

    #[rstest(
        a,
        b,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
        )
    )]
    fn test_min(a: EntryEnd, b: EntryEnd, expected: EntryEnd) {
        assert_eq!(EntryEnd::min(a, b), expected)
    }

    #[rstest(
        a,
        b,
        expected,
        case(
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
        ),
        case(
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::generate_datetime("2000-01-01T12:00:00+00:00")),
        )
    )]
    fn test_max(a: EntryEnd, b: EntryEnd, expected: EntryEnd) {
        assert_eq!(EntryEnd::max(a, b), expected)
    }
}
