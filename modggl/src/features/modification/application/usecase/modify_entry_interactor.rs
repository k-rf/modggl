use std::sync::Arc;
use std::{thread, time::Duration};

use async_trait::async_trait;

use crate::utils;

use super::domain::entry::{EntryList, EntrySince, EntryUntil};
use super::domain::{ActionType, EntryLogger, EntryReviser, ResultModified, ReviserStatus};
use super::port::incoming::{ModifyEntryCommand, ModifyEntryUsecase};
use super::port::outgoing::{
    EntryModifiedOutputData, EntryModifiedPresenterPort, EntryRepositoryPort,
};

pub struct ModifyEntryInteractor {
    pub entry_repository_port: Arc<dyn EntryRepositoryPort + Sync + Send>,
    pub entry_modified_presenter_port: Arc<dyn EntryModifiedPresenterPort + Sync + Send>,
}

#[async_trait]
impl ModifyEntryUsecase for ModifyEntryInteractor {
    async fn execute(&self, command: ModifyEntryCommand) -> String {
        let since = EntrySince::new(command.since);
        let until = EntryUntil::new(command.until);
        let entries = self.entry_repository_port.get(since, until).await;

        let mut modification_list = EntryList::new(vec![]);
        let mut deletion_list = EntryList::new(vec![]);

        let mut reviser = EntryReviser::new();

        for entry in entries.value.into_iter() {
            if let Ok(e) = reviser.insert(entry) {
                if let ReviserStatus::Full = e {
                    if let Ok(e) = reviser.merge() {
                        modification_list.upsert(e.merged);
                        deletion_list.upsert(e.deletable);
                        continue;
                    }

                    match reviser.modify() {
                        ResultModified::Modified(modified) => {
                            modification_list.upsert(modified);
                        }
                        ResultModified::Unnecessary => {}
                        ResultModified::InvalidOrder => {
                            let message =
                                String::from("The retrieved values are in an invalid order.");

                            log::error!("{}", message);
                            self.entry_modified_presenter_port
                                .execute(EntryModifiedOutputData {
                                    message: message.clone(),
                                })
                                .await;

                            return message;
                        }
                        ResultModified::NotDetermine(entry) => {
                            let message = String::new()
                                + "Please modify an entry manually in "
                                + &format!(
                                    "{}.",
                                    utils::format_datetime(&entry.period.start.value),
                                );

                            log::error!("{}", message);
                            self.entry_modified_presenter_port
                                .execute(EntryModifiedOutputData {
                                    message: message.clone(),
                                })
                                .await;

                            return message;
                        }
                    }
                } else {
                    continue;
                };
            }
        }

        let info_logger = EntryLogger(log::Level::Info);

        for entry in modification_list.value.into_iter() {
            info_logger(&entry, ActionType::Modify);

            thread::sleep(Duration::from_millis(500)); // Toggl API 側の DDoS 判定を回避する
            self.entry_repository_port.modify(entry).await;
        }

        for entry in deletion_list.value.into_iter() {
            info_logger(&entry, ActionType::Delete);

            thread::sleep(Duration::from_millis(500)); // Toggl API 側の DDoS 判定を回避する
            self.entry_repository_port.delete(entry).await;
        }

        let message = String::from("Modification is completed.");
        log::info!("{}", message);
        self.entry_modified_presenter_port
            .execute(EntryModifiedOutputData {
                message: message.clone(),
            })
            .await;

        message
    }
}

#[cfg(test)]
mod tests {
    use tokio;

    use crate::features::modification::application::port::incoming::ModifyEntryCommand;
    use crate::features::modification::presenter::EntryModifiedSlackPresenter;
    use crate::features::modification::repository::MockTogglRepository;
    use crate::services::LoggerService;
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_modify_entry() {
        LoggerService::init();
        dotenv::from_filename(".env.test.local").ok();

        let interactor = ModifyEntryInteractor {
            entry_repository_port: Arc::new(MockTogglRepository::new()),
            entry_modified_presenter_port: Arc::new(EntryModifiedSlackPresenter::new()),
        };

        interactor
            .execute(ModifyEntryCommand {
                since: utils::generate_datetime("2022-02-01T00:00:00+00:00"),
                until: utils::generate_datetime("2022-02-11T00:00:00+00:00"),
            })
            .await;

        assert!(true)
    }
}
