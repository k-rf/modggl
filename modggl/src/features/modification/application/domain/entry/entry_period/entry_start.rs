use std::cmp;

use chrono::{DateTime, Utc};

use super::EntryEnd;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct EntryStart {
    pub value: DateTime<Utc>,
}

impl EntryStart {
    pub fn new(value: DateTime<Utc>) -> Self {
        EntryStart { value }
    }
}

impl cmp::PartialEq<EntryEnd> for EntryStart {
    fn eq(&self, other: &EntryEnd) -> bool {
        self.value == other.value
    }
}

impl cmp::PartialOrd<EntryEnd> for EntryStart {
    fn partial_cmp(&self, other: &EntryEnd) -> Option<cmp::Ordering> {
        if self.lt(other) {
            Some(cmp::Ordering::Less)
        } else if self.gt(other) {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Equal)
        }
    }

    fn le(&self, other: &EntryEnd) -> bool {
        self.value <= other.value
    }

    fn lt(&self, other: &EntryEnd) -> bool {
        self.value < other.value
    }

    fn ge(&self, other: &EntryEnd) -> bool {
        self.value >= other.value
    }

    fn gt(&self, other: &EntryEnd) -> bool {
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
            EntryStart::new(utils::date_generator("2021-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2021-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryStart::new(utils::date_generator("2021-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2022-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_equal(this: EntryStart, that: EntryEnd, expected: bool) {
        assert_eq!(this == that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryStart::new(utils::date_generator("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryStart::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_less_than(this: EntryStart, that: EntryEnd, expected: bool) {
        assert_eq!(this < that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryStart::new(utils::date_generator("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryStart::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryStart::new(utils::date_generator("2100-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_less_than_or_equal_to(this: EntryStart, that: EntryEnd, expected: bool) {
        assert_eq!(this <= that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryStart::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("1900-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryStart::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_greater_than(this: EntryStart, that: EntryEnd, expected: bool) {
        assert_eq!(this > that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryStart::new(utils::date_generator("2100-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryStart::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            true
        ),
        case(
            EntryStart::new(utils::date_generator("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            false
        )
    )]
    fn test_greater_than_or_equal_to(this: EntryStart, that: EntryEnd, expected: bool) {
        assert_eq!(this >= that, expected)
    }

    #[rstest(
        this,
        that,
        expected,
        case(
            EntryStart::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            cmp::Ordering::Equal
        ),
        case(
            EntryStart::new(utils::date_generator("1900-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            cmp::Ordering::Less
        ),
        case(
            EntryStart::new(utils::date_generator("2000-01-01T12:00:00+00:00")),
            EntryEnd::new(utils::date_generator("1900-01-01T12:00:00+00:00")),
            cmp::Ordering::Greater
        )
    )]
    fn test_compare(this: EntryStart, that: EntryEnd, expected: cmp::Ordering) {
        assert_eq!(this.partial_cmp(&that).unwrap(), expected)
    }
}
