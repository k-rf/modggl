use crate::utils::format_datetime;

use super::entry::{Entry, EntryRelation};

pub fn entry_relation_logger(level: log::Level) -> impl Fn(&EntryRelation, &Entry, &Entry) {
    move |relation: &EntryRelation, first: &Entry, second: &Entry| {
        let msg = format!(
            "{}--{}, {}--{}, {:?}",
            format_datetime(&first.period.start.value),
            format_datetime(&first.period.end.value),
            format_datetime(&second.period.start.value),
            format_datetime(&second.period.end.value),
            relation,
        );

        match level {
            log::Level::Info => {
                log::info!("{}", msg);
            }
            log::Level::Error => {
                log::error!("{}", msg);
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
pub enum ActionType {
    Modify,
    Delete,
}

pub fn entry_logger(level: log::Level) -> impl Fn(&Entry, ActionType) {
    move |entry: &Entry, action: ActionType| {
        let msg = format!(
            "{}--{}, {:?}",
            format_datetime(&entry.period.start.value),
            format_datetime(&entry.period.end.value),
            action
        );

        match level {
            log::Level::Info => {
                log::info!("{}", msg);
            }
            log::Level::Error => {
                log::error!("{}", msg);
            }
            _ => (),
        }
    }
}
