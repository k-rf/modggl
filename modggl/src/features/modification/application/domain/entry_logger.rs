use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;

use super::entry::{Entry, EntryRelation};

fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.with_timezone(&Tokyo)
        .format("%Y-%m-%dT%H:%M:%S%z")
        .to_string()
}

pub fn logger(level: log::Level) -> impl Fn(&EntryRelation, &Entry, &Entry) {
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
