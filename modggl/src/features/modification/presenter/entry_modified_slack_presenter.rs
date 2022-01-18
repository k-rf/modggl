use async_trait::async_trait;

use crate::features::modification::application::port::outgoing::EntryModifiedPresenterPort;

use super::application::port::outgoing::EntryModifiedOutputData;
pub struct EntryModifiedSlackPresenter {}

impl EntryModifiedSlackPresenter {
    pub fn new() -> Self {
        EntryModifiedSlackPresenter {}
    }
}

#[async_trait]
impl EntryModifiedPresenterPort for EntryModifiedSlackPresenter {
    async fn execute(&self, value: EntryModifiedOutputData) {
        log::info!("{}", value.message);
    }
}
