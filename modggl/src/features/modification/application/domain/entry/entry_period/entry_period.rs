use super::{EntryDuration, EntryEnd, EntryRelation, EntryStart};

#[derive(PartialEq, Eq)]
pub struct EntryPeriod {
    pub start: EntryStart,
    pub end: EntryEnd,
    pub duration: EntryDuration,
}

impl EntryPeriod {
    pub fn new(props: Props) -> Self {
        let duration = props.end.value.timestamp() - props.start.value.timestamp();

        let period = EntryPeriod {
            start: props.start,
            end: props.end,
            duration: EntryDuration::new(duration),
        };

        period.validate();

        period
    }

    fn validate(&self) {
        if self.duration.value < 0 {
            panic!("EntryStart <= EntryEnd");
        }
    }

    pub fn compare(&self, other: EntryPeriod) -> EntryRelation {
        if self.start == other.start && self.end == other.end {
            return EntryRelation::Equivalent;
        }

        if self.start < other.start && self.end <= other.start && self.end < other.end {
            return EntryRelation::Less;
        }

        if other.start < self.start && other.end <= self.start && other.end < self.end {
            return EntryRelation::Greater;
        }

        if self.start < other.start && other.start < self.end && self.end < other.end {
            return EntryRelation::LessOverlap;
        }

        if other.start < self.start && self.start < other.end && other.end < self.end {
            return EntryRelation::GreaterOverlap;
        }

        if self.start < other.start && other.end <= self.end {
            return EntryRelation::LessOuter;
        }

        if self.start == other.start && other.end < self.end {
            return EntryRelation::GreaterOuter;
        }

        if self.start == other.start && self.end < other.end {
            return EntryRelation::LessInner;
        }

        if other.start < self.start && self.end <= other.end {
            return EntryRelation::GreaterInner;
        }

        panic!("Cannot compare");
    }
}

pub struct Props {
    pub start: EntryStart,
    pub end: EntryEnd,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::utils;

    use super::*;

    #[rstest(
        start,
        end,
        case(
            EntryStart::new(utils::date_generator("2022-01-01T09:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2022-01-01T12:00:00+00:00")),
        ),
        case(
            EntryStart::new(utils::date_generator("2022-01-01T09:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2022-01-01T09:00:00+00:00")),
        ),
        #[should_panic]
        case(
            EntryStart::new(utils::date_generator("2022-01-01T12:00:01+00:00")),
            EntryEnd::new(utils::date_generator("2022-01-01T12:00:00+00:00")),
        ),
        #[should_panic]
        case(
            EntryStart::new(utils::date_generator("2022-01-02T09:00:00+00:00")),
            EntryEnd::new(utils::date_generator("2022-01-01T12:00:00+00:00")),
        ),
    )]
    fn test_new(start: EntryStart, end: EntryEnd) {
        EntryPeriod::new(Props { start, end });
    }

    #[rstest(first, second, expected,
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::Equivalent
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
            }),
            EntryRelation::Equivalent
        ),

        // Greater
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T06:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T08:00:00+00:00")),
            }),
            EntryRelation::Greater
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T06:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
            }),
            EntryRelation::Greater
        ),

        // Less
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T15:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T18:00:00+00:00")),
            }),
            EntryRelation::Less
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T15:00:00+00:00")),
            }),
            EntryRelation::Less
        ),

        // Overlap
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T10:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T13:00:00+00:00")),
            }),
            EntryRelation::LessOverlap
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T10:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T13:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::GreaterOverlap
        ),

        // LessOuter
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T10:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T11:00:00+00:00")),
            }),
            EntryRelation::LessOuter
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::LessOuter
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T10:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::LessOuter
        ),

        // GreaterOuter
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
            }),
            EntryRelation::GreaterOuter
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T11:00:00+00:00")),
            }),
            EntryRelation::GreaterOuter
        ),

        // LessInner
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::LessInner
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T11:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::LessInner
        ),

        // GreaterInner
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T10:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T11:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::GreaterInner
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T10:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::GreaterInner
        ),
        case(
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryPeriod::new(Props {
                start: EntryStart::new(utils::date_generator("2022-01-15T09:00:00+00:00")),
                end: EntryEnd::new(utils::date_generator("2022-01-15T12:00:00+00:00")),
            }),
            EntryRelation::GreaterInner
        ),
    )]
    fn test_compare(first: EntryPeriod, second: EntryPeriod, expected: EntryRelation) {
        assert_eq!(first.compare(second), expected)
    }
}
