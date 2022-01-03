use std::fmt::Error;

use super::{
    entry_client::EntryClient, entry_description::EntryDescription, entry_id::EntryId,
    entry_period::EntryPeriod, entry_project::EntryProject, entry_tag_list::EntryTagList,
    entry_updated::EntryUpdated,
};

#[derive(PartialEq, Eq)]
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

    pub fn merge(&self, other: Entry) -> Result<Self, &str> {
        Entry::new(Props {
            client: self.client,
            description: self.description,
            id: self.id,
            period: self.period,
            project: self.project,
            tags: self.tags,
            updated_at: self.updated_at,
        })
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
    use crate::utils::date_generator;

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
                .start(date_generator("2000-01-01T12:00:00+00:00"))
                .end(date_generator("2000-01-01T15:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(date_generator("2000-01-01T16:00:00+00:00"))
                .end(date_generator("2000-01-01T18:00:00+00:00"))
                .build(),
            EntryBuilder::new()
                .start(date_generator("2000-01-01T12:00:00+00:00"))
                .end(date_generator("2000-01-01T18:00:00+00:00"))
                .build()
        ),
    )]
    fn test_merge(a: Entry, b: Entry, expected: Entry) {
        assert!(a.merge(b) == Ok(expected));
    }
}
