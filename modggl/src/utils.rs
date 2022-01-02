use chrono::DateTime;
use chrono::Utc;

pub fn date_generator(value: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from(DateTime::parse_from_rfc3339(value).unwrap())
}
