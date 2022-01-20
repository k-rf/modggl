use async_trait::async_trait;

use super::ModifyEntryCommand;

#[async_trait]
pub trait ModifyEntryUsecase {
    async fn execute(&self, command: ModifyEntryCommand) -> String;
}
