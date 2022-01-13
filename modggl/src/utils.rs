use chrono::Utc;
use chrono::{Date, DateTime, NaiveDate};

pub fn datetime_generator(value: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from(DateTime::parse_from_rfc3339(value).unwrap())
}

pub fn date_generator(value: &str) -> Date<Utc> {
    Date::<Utc>::from_utc(NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap(), Utc)
}
