pub use entry_logger::logger as EntryLogger;
pub use entry_reviser::{EntryReviser, ResultCompared, ReviserStatus};

pub mod entry;
mod entry_logger;
mod entry_reviser;
