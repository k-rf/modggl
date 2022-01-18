use std::sync::Arc;

use async_trait::async_trait;

use super::domain::entry::{EntryList, EntrySince, EntryUntil};
use super::domain::{ActionType, EntryLogger, EntryReviser, ResultModified, ReviserStatus};
use super::port::incoming::ModifyEntryCommand;
use super::port::incoming::ModifyEntryUsecase;
use super::port::outgoing::EntryTogglRepositoryPort;

pub struct ModifyEntryInteractor {
    pub toggl_repository_port: Arc<dyn EntryTogglRepositoryPort + Sync + Send>,
}

#[async_trait]
impl ModifyEntryUsecase for ModifyEntryInteractor {
    async fn execute(&self, command: ModifyEntryCommand) {
        let since = EntrySince::new(command.since);
        let until = EntryUntil::new(command.until);
        let entries = self.toggl_repository_port.get(since, until).await;

        let mut modification_list = EntryList::new(vec![]);
        let mut deletion_list = EntryList::new(vec![]);

        let mut reviser = EntryReviser::new();

        for entry in entries.value.into_iter() {
            if let Ok(e) = reviser.insert(entry) {
                match e {
                    ReviserStatus::Full => {
                        if let Ok(e) = reviser.merge() {
                            modification_list.upsert(e.merged);
                            deletion_list.upsert(e.deletable);
                            continue;
                        }

                        match reviser.modify() {
                            ResultModified::Modified(modified) => {
                                modification_list.upsert(modified)
                            }
                            // TODO: Presenter を実装し、外部に通知できるようにする
                            // 修正中にエラーパターンが出てきたら、ここで Presenter を呼び出す
                            _ => (),
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        let info_logger = EntryLogger(log::Level::Info);

        for entry in modification_list.value.into_iter() {
            info_logger(&entry, ActionType::Modify);
            self.toggl_repository_port.modify(entry).await;
        }

        for entry in deletion_list.value.into_iter() {
            info_logger(&entry, ActionType::Delete);
            self.toggl_repository_port.delete(entry).await;
        }

        // TODO: Presenter を実装し、外部に通知できるようにする
        // 成功した場合はここで通知するようにする
        log::info!("Modification is completed.");
    }
}

#[cfg(test)]
mod tests {
    use tokio;

    use crate::features::modification::application::port::incoming::ModifyEntryCommand;
    use crate::features::modification::repository::EntryTogglRepository;
    use crate::services::LoggerService;
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_modify_entry() {
        LoggerService::init();
        dotenv::from_filename(".env.local").ok();

        let interactor = ModifyEntryInteractor {
            toggl_repository_port: Arc::new(EntryTogglRepository::new()),
        };

        interactor
            .execute(ModifyEntryCommand {
                since: utils::date_generator("2022-02-01"),
                until: utils::date_generator("2022-02-11"),
            })
            .await;

        assert!(true)
    }
}
