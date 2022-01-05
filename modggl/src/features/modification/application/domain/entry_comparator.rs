use std::collections::VecDeque;

use super::entry::{Entry, EntryRelation};

const CAPACITY: usize = 2;

#[derive(Debug, PartialEq, Eq)]
enum ResultCompared {
    Relation(EntryRelation),
    Merged(Entry),
}

struct EntryComparator {
    value: VecDeque<Entry>,
}

impl EntryComparator {
    pub fn new() -> Self {
        EntryComparator {
            value: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn push(&mut self, other: Entry) {
        if self.value.len() == CAPACITY {
            panic!("Capacity over");
        }

        self.value.push_back(other);
    }

    pub fn compare(&mut self) -> ResultCompared {
        if self.len() != CAPACITY {
            panic!("Cannot compare");
        }

        let first = self.value.pop_front().unwrap();
        let second = self.value.pop_front().unwrap();

        match first.merge(second) {
            Ok(merged) => ResultCompared::Merged(merged),
            Err(second) => {
                self.push(second.clone());
                ResultCompared::Relation(first.period.compare(second.period))
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.len() == CAPACITY
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{features::modification::application::domain::entry::EntryBuilder, utils};

    use super::*;

    #[rstest(
        input,
        expected,
        case(
            vec![],
            0
        ),
        case(
            vec![EntryBuilder::new().build()],
            1
        ),
        case(
            vec![EntryBuilder::new().build(), EntryBuilder::new().build(), ],
            2
        ),
        #[should_panic]
        case(
            vec![EntryBuilder::new().build(), EntryBuilder::new().build(), EntryBuilder::new().build()],
            0
        ),
    )]
    fn test_push(input: Vec<Entry>, expected: usize) {
        let mut comparator = EntryComparator::new();

        for entry in input.into_iter() {
            comparator.push(entry);
        }

        assert_eq!(comparator.len(), expected);
    }

    #[rstest(
        input,
        expected,
        case(
            vec![],
            false,
        ),
        case(
            vec![EntryBuilder::new().build()],
            false,
        ),
        case(
            vec![EntryBuilder::new().build(), EntryBuilder::new().build()],
            true,
        ),
    )]
    fn test_is_full(input: Vec<Entry>, expected: bool) {
        let mut comparator = EntryComparator::new();

        for entry in input.into_iter() {
            comparator.push(entry);
        }

        assert_eq!(comparator.is_full(), expected);
    }

    #[rstest(
        input,
        expected,
        #[should_panic]
        case(
            vec![],
            ResultCompared::Relation(EntryRelation::Equivalent),
        ),
        #[should_panic]
        case(
            vec![EntryBuilder::new().build()],
            ResultCompared::Relation(EntryRelation::Equivalent),
        ),
        case(
            vec![
                EntryBuilder::new()
                    .start(utils::date_generator("2000-01-01T09:00:00+00:00"))
                    .end(utils::date_generator("2000-01-01T12:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .start(utils::date_generator("2000-01-01T15:00:00+00:00"))
                    .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                    .build()
            ],
            ResultCompared::Merged(
                EntryBuilder::new()
                    .start(utils::date_generator("2000-01-01T09:00:00+00:00"))
                    .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                    .build()
            ),
        ),
        case(
            vec![
                EntryBuilder::new().client("abc").build(),
                EntryBuilder::new().client("xyz").build(),
            ],
            ResultCompared::Relation(EntryRelation::Equivalent),
        ),
    )]
    fn test_compare(input: Vec<Entry>, expected: ResultCompared) {
        let mut comparator = EntryComparator::new();

        for entry in input.into_iter() {
            comparator.push(entry);
        }

        assert_eq!(comparator.compare(), expected);
    }
}
