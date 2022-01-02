use super::{
    entry_client::EntryClient, entry_description::EntryDescription, entry_id::EntryId,
    entry_period::EntryPeriod, entry_project::EntryProject, entry_tag_list::EntryTagList,
    entry_updated::EntryUpdated,
};

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
    use chrono::Utc;
    use rstest::rstest;

    use crate::features::modification::application::domain::entry::entry_period::EntryEnd;
    use crate::features::modification::application::domain::entry::entry_period::EntryPeriodProps;
    use crate::features::modification::application::domain::entry::entry_period::EntryStart;
    use crate::features::modification::application::domain::entry::entry_tag::EntryTag;

    use super::*;

    fn generate_entry(
        client: String,
        project: String,
        description: String,
        tags: Vec<EntryTag>,
    ) -> Entry {
        Entry::new(Props {
            client: EntryClient::new(client),
            description: EntryDescription::new(description),
            id: EntryId::new(String::from("abc")),
            period: EntryPeriod::new(EntryPeriodProps {
                start: EntryStart::new(Utc::now()),
                end: EntryEnd::new(Utc::now()),
            }),
            project: EntryProject::new(project),
            tags: EntryTagList::new(tags),
            updated_at: EntryUpdated::new(Utc::now()),
        })
    }

    #[rstest(
        a,
        b,
        expected,
        case(
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            true
        ),
        case(
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc")), EntryTag::new(String::from("xyz"))]),
            false
        ),
        case(
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("xyz"))]),
            false
        ),
        case(
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            generate_entry(String::from("xyz"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("xyz"))]),
            false
        ),
        case(
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            generate_entry(String::from("abc"), String::from("xyz"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            false
        ),
        case(
            generate_entry(String::from("abc"), String::from("abc"), String::from("abc"), vec![EntryTag::new(String::from("abc"))]),
            generate_entry(String::from("abc"), String::from("abc"), String::from("xyz"), vec![EntryTag::new(String::from("abc"))]),
            false
        )
    )]
    fn test_test_be_same(a: Entry, b: Entry, expected: bool) {
        assert!(a.is_same(&b) == expected)
    }
}
