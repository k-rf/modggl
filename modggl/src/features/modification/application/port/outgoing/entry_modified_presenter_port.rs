use async_trait::async_trait;

use super::EntryModifiedOutputData;

#[async_trait]
pub trait EntryModifiedPresenterPort {
    async fn execute(&self, value: EntryModifiedOutputData);
}
