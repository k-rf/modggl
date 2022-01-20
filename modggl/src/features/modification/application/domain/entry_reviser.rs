use std::collections::VecDeque;

use super::entry::{Entry, EntryRelation, ResultMerged};
use super::EntryRelationLogger;

const CAPACITY: usize = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct ResultCompared {
    pub relation: EntryRelation,
    pub first: Entry,
    pub second: Entry,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResultModified {
    Modified(Entry),
    Unnecessary,
    NotDetermine,
    InvalidOrder,
}

pub enum ReviserStatus {
    Full,
    NotEmpty,
}

pub struct EntryReviser {
    value: VecDeque<Entry>,
}

impl EntryReviser {
    pub fn new() -> Self {
        EntryReviser {
            value: VecDeque::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.len() == CAPACITY
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn insert(&mut self, other: Entry) -> Result<ReviserStatus, Entry> {
        if self.is_full() {
            Err(other)
        } else {
            self.value.push_back(other);

            if self.is_full() {
                Ok(ReviserStatus::Full)
            } else {
                Ok(ReviserStatus::NotEmpty)
            }
        }
    }

    pub fn modify(&mut self) -> ResultModified {
        let info_logger = EntryRelationLogger(log::Level::Info);
        let error_logger = EntryRelationLogger(log::Level::Error);

        if !self.is_full() {
            panic!("Can not modify.");
        }

        let first = self.value.pop_front().unwrap();
        let second = self.value.pop_front().unwrap();

        if first.is_same(&second) {
            panic!("Can not modify.");
        }

        self.insert(second.clone()).unwrap();

        let relation = first.period.compare(&second.period);
        match relation {
            EntryRelation::Less | EntryRelation::LessOuter | EntryRelation::LessOverlap => {
                info_logger(&relation, &first, &second);
                if first.period.end == second.period.start {
                    ResultModified::Unnecessary
                } else {
                    ResultModified::Modified(first.modify_period_end(second.period.start))
                }
            }
            EntryRelation::Greater
            | EntryRelation::GreaterOverlap
            | EntryRelation::GreaterInner => {
                error_logger(&relation, &first, &second);
                ResultModified::InvalidOrder
            }
            EntryRelation::Equivalent | EntryRelation::GreaterOuter | EntryRelation::LessInner => {
                error_logger(&relation, &first, &second);
                ResultModified::NotDetermine
            }
        }
    }

    pub fn merge(&mut self) -> Result<ResultMerged, ()> {
        if !self.is_full() {
            panic!("Cannot merge entries.");
        }

        let first = self.value.pop_front().unwrap();
        let second = self.value.pop_front().unwrap();

        match first.merge(second) {
            Ok(e) => {
                self.insert(e.merged.clone()).unwrap();
                Ok(e)
            }
            Err(second) => {
                self.insert(first).unwrap();
                self.insert(second).unwrap();
                Err(())
            }
        }
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
        let mut reviser = EntryReviser::new();

        for entry in input.into_iter() {
            reviser.insert(entry).unwrap();
        }

        assert_eq!(reviser.len(), expected);
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
        let mut reviser = EntryReviser::new();

        for entry in input.into_iter() {
            reviser.insert(entry).unwrap();
        }

        assert_eq!(reviser.is_full(), expected);
    }

    #[rstest(
        input,
        expected,
        #[should_panic]
        case(
            vec![],
            EntryBuilder::new().build(),
        ),
        #[should_panic]
        case(
            vec![EntryBuilder::new().build()],
            EntryBuilder::new().build(),
        ),
        #[should_panic]
        case(
            vec![
                EntryBuilder::new().build(),
                EntryBuilder::new().build(),
            ],
            EntryBuilder::new().build(),
        ),
        case(
            vec![
                EntryBuilder::new()
                    .client("abc")
                    .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T18:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .client("xyz")
                    .start(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T15:00:00+00:00"))
                    .build(),
            ],
            EntryBuilder::new()
                .client("abc")
                .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                .end(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                .build(),
        ),
        case(
            vec![
                EntryBuilder::new()
                    .client("abc")
                    .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T13:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .client("xyz")
                    .start(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T15:00:00+00:00"))
                    .build(),
            ],
            EntryBuilder::new()
                .client("abc")
                .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                .end(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                .build(),
        ),
    )]
    fn test_modify(input: Vec<Entry>, expected: Entry) {
        let mut reviser = EntryReviser::new();

        for entry in input.into_iter() {
            reviser.insert(entry).unwrap();
        }

        if let ResultModified::Modified(actual) = reviser.modify() {
            assert_eq!(actual, expected);
        } else {
            panic!("Fail");
        }
    }

    #[rstest(
        input,
        expected,
        case(
            vec![
                EntryBuilder::new()
                    .client("abc")
                    .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .client("xyz")
                    .start(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T15:00:00+00:00"))
                    .build(),
            ],
            ResultModified::Unnecessary,
        ),
        case(
            vec![
                EntryBuilder::new()
                    .client("xyz")
                    .start(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T15:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .client("abc")
                    .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .build(),
            ],
            ResultModified::InvalidOrder,
        ),
        case(
            vec![
                EntryBuilder::new()
                    .client("abc")
                    .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .build(),
                EntryBuilder::new()
                    .client("xyz")
                    .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T17:00:00+00:00"))
                    .build(),
            ],
            ResultModified::NotDetermine(
                EntryBuilder::new()
                    .client("abc")
                    .start(utils::generate_datetime("2000-01-01T09:00:00+00:00"))
                    .end(utils::generate_datetime("2000-01-01T12:00:00+00:00"))
                    .build(),
            ),
        ),
    )]
    fn test_modify_err(input: Vec<Entry>, expected: ResultModified) {
        let mut reviser = EntryReviser::new();

        for entry in input.into_iter() {
            reviser.insert(entry).unwrap();
        }

        assert_eq!(reviser.modify(), expected);
    }
}
