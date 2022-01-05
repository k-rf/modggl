use super::{
    entry_client::EntryClient,
    entry_description::EntryDescription,
    entry_id::EntryId,
    entry_period::{EntryEnd, EntryPeriod, EntryPeriodProps, EntryStart},
    entry_project::EntryProject,
    entry_tag_list::EntryTagList,
    entry_updated::EntryUpdated,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    pub client: EntryClient,
    pub description: EntryDescription,
    pub id: EntryId,
    pub period: EntryPeriod,
    pub project: EntryProject,
    pub tags: EntryTagList,
    pub updated_at: EntryUpdated,
}

impl Entry {
    pub fn new(props: Props) -> Self {
        Entry {
            client: props.client,
            description: props.description,
            id: props.id,
            period: props.period,
            project: props.project,
            tags: props.tags,
            updated_at: props.updated_at,
        }
    }

    pub fn is_same(&self, other: &Entry) -> bool {
        self.client == other.client
            && self.project == other.project
            && self.description == other.description
            && self.tags == other.tags
    }

    pub fn merge(&self, other: Entry) -> Result<Self, Entry> {
        if self.is_same(&other) {
            let EntryPeriod {
                start: self_start,
                end: self_end,
                ..
            } = self.period;
            let EntryPeriod {
                start: other_start,
                end: other_end,
                ..
            } = other.period;

            Ok(Entry::new(Props {
                client: self.client.clone(),
                description: self.description.clone(),
                id: self.id.clone(),
                period: EntryPeriod::new(EntryPeriodProps {
                    start: EntryStart::min(self_start, other_start),
                    end: EntryEnd::max(self_end, other_end),
                }),
                project: self.project.clone(),
                tags: self.tags.clone(),
                updated_at: EntryUpdated::min(self.updated_at, other.updated_at),
            }))
        } else {
            Err(other)
        }
    }
}

pub struct Props {
    client: EntryClient,
    description: EntryDescription,
    id: EntryId,
    period: EntryPeriod,
    project: EntryProject,
    tags: EntryTagList,
    updated_at: EntryUpdated,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::features::modification::application::domain::entry::test_utils::EntryBuilder;
    use crate::utils;

    use super::*;

    #[rstest(
        a,
        b,
        expected,
        case(
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc"])
                .build(),
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc"])
                .build(),
            true
        ),
        case(
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc"])
                .build(),
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc", "xyz"])
                .build(),
            false
        ),
        case(
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc"])
                .build(),
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["xyz"])
                .build(),
            false
        ),
        case(
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc"])
                .build(),
            EntryBuilder::new()
                .client("xyz").project("abc").description("abc")
                .tags(vec!["xyz"])
                .build(),
            false
        ),
        case(
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc"])
                .build(),
            EntryBuilder::new()
                .client("abc").project("xyz").description("abc")
                .tags(vec!["abc"])
                .build(),
            false
        ),
        case(
            EntryBuilder::new()
                .client("abc").project("abc").description("abc")
                .tags(vec!["abc"])
                .build(),
            EntryBuilder::new()
                .client("abc").project("abc").description("xyz")
                .tags(vec!["abc"])
                .build(),
            false
        )
    )]
    fn test_is_same(a: Entry, b: Entry, expected: bool) {
        assert_eq!(a.is_same(&b), expected)
    }

    #[rstest(
        a,
        b,
        expected,
        case(
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T16:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T15:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build()
        ),
        case(
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T15:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T16:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build()
        ),
        case(
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T15:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T13:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build()
        ),
        case(
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T10:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T15:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T14:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T10:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T15:00:00+00:00"))
                .build()
        ),
        case(
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T15:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T10:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(utils::date_generator("2000-01-01T10:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build()
        )
        ::trace
    )]
    fn test_merge(a: Entry, b: Entry, expected: Entry) {
        if let Ok(actual) = a.merge(b) {
            print!("{:?}", actual);
            assert!(actual == expected);
        } else {
            panic!("Fail")
        }
    }

    #[rstest(
        a,
        b,
        expected,
        case(
            EntryBuilder::new()
                .project("abc")
                .start(utils::date_generator("2000-01-01T12:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T15:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .project("xyz")
                .start(utils::date_generator("2000-01-01T16:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .project("xyz")
                .start(utils::date_generator("2000-01-01T16:00:00+00:00"))
                .end(utils::date_generator("2000-01-01T18:00:00+00:00"))
                .build(),
        ),
    )]
    fn test_merge_err(a: Entry, b: Entry, expected: Entry) {
        if let Err(actual) = a.merge(b) {
            assert_eq!(actual, expected);
        } else {
            panic!("Fail")
        }
    }
}
