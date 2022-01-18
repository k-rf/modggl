pub use entry_logger::{
    entry_logger as EntryLogger, entry_relation_logger as EntryRelationLogger, ActionType,
};
pub use entry_reviser::{EntryReviser, ResultModified, ReviserStatus};

pub mod entry;
mod entry_logger;
mod entry_reviser;
