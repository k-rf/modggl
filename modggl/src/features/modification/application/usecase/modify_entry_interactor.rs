use std::sync::Arc;

use async_trait::async_trait;

use super::domain::entry::{EntrySince, EntryUntil};
use super::modify_entry_command::ModifyEntryCommand;
use super::modify_entry_usecase::ModifyEntryUsecase;
use super::toggl_repository_port::TogglRepositoryPort;

pub struct ModifyEntryInteractor {
    pub toggl_repository_port: Arc<dyn TogglRepositoryPort + Sync + Send>,
}

#[async_trait]
impl ModifyEntryUsecase for ModifyEntryInteractor {
    async fn execute(&self, command: ModifyEntryCommand) {
        let since = EntrySince::new(command.since);
        let until = EntryUntil::new(command.until);
        let entries = self.toggl_repository_port.get(since, until).await;

        // self.toggl_repository_port.modify();
    }
}
