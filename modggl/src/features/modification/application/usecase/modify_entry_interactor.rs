use std::collections::VecDeque;
use std::sync::Arc;

use async_trait::async_trait;

use super::domain::entry::{Entry, EntryId, EntrySince, EntryUntil};
use super::port::incoming::ModifyEntryCommand;
use super::port::incoming::ModifyEntryUsecase;
use super::port::outgoing::TogglRepositoryPort;

pub struct ModifyEntryInteractor {
    pub toggl_repository_port: Arc<dyn TogglRepositoryPort + Sync + Send>,
}

#[async_trait]
impl ModifyEntryUsecase for ModifyEntryInteractor {
    async fn execute(&self, command: ModifyEntryCommand) {
        let since = EntrySince::new(command.since);
        let until = EntryUntil::new(command.until);
        let entries = self.toggl_repository_port.get(since, until).await;

        // Todo: サイズ制限できないので、自作の Comparator が必要かも
        let mut comparator: VecDeque<Entry> = VecDeque::with_capacity(2);

        for entry in entries.value.into_iter() {
            comparator.push_back(entry);
        }

        print!("{:?}\n", comparator.capacity());
        // self.toggl_repository_port.modify();
    }
}

#[cfg(test)]
mod tests {
    use tokio;

    use crate::features::modification::application::port::incoming::ModifyEntryCommand;
    use crate::features::modification::repository::TogglRepository;
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_modify_entry() {
        let interactor = ModifyEntryInteractor {
            toggl_repository_port: Arc::new(TogglRepository {}),
        };

        interactor
            .execute(ModifyEntryCommand {
                since: utils::date_generator("2000-01-01T00:00:00+00:00"),
                until: utils::date_generator("2000-01-01T00:00:00+00:00"),
            })
            .await;
    }
}
