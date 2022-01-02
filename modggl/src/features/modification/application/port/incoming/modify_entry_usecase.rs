use async_trait::async_trait;

use super::modify_entry_command::ModifyEntryCommand;

#[async_trait]
pub trait ModifyEntryUsecase {
    async fn execute(&self, command: ModifyEntryCommand);
}
