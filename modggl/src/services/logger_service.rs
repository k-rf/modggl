use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub struct LoggerService {}

impl LoggerService {
    pub fn init() {
        let mut builder = Builder::from_default_env();

        builder
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{} {}] {}",
                    Utc::now()
                        .with_timezone(&Tokyo)
                        .format("%Y-%m-%dT%H:%M:%S%z"),
                    record.level(),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Info)
            .init();
    }
}
