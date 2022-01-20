use chrono::{Date, DateTime, Duration, NaiveDate, Utc};
use chrono_tz::Asia::Tokyo;

pub fn generate_datetime(value: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from(DateTime::parse_from_rfc3339(value).unwrap())
}

#[allow(dead_code)]
pub fn generate_date(value: &str) -> Date<Utc> {
    Date::<Utc>::from_utc(NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap(), Utc)
}

pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.with_timezone(&Tokyo)
        .format("%Y-%m-%dT%H:%M:%S%z")
        .to_string()
}

pub fn date_divider(since: Date<Utc>, until: Date<Utc>) -> Vec<(Date<Utc>, Date<Utc>)> {
    let mut since_list: Vec<Date<Utc>> = vec![since];
    let mut until_list: Vec<Date<Utc>> = vec![];

    let year = Duration::days(364);
    let day = Duration::days(1);

    let mut next_since = since;
    loop {
        let next_until = next_since + year;
        next_since = next_until + day;

        if until < next_since {
            break;
        }

        until_list.push(next_until);
        since_list.push(next_since);
    }
    until_list.push(until);

    since_list
        .into_iter()
        .zip(until_list.into_iter())
        .map(|(s, u)| (s, u))
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        since,
        until,
        expected,
        case(
            generate_date("2022-01-01"),
            generate_date("2022-08-31"),
            vec![(generate_date("2022-01-01"), generate_date("2022-08-31"))],
        ),
        case(
            generate_date("2022-01-01"),
            generate_date("2022-12-31"),
            vec![(generate_date("2022-01-01"), generate_date("2022-12-31"))],
        ),
        case(
            generate_date("2022-01-01"),
            generate_date("2023-08-31"),
            vec![
                (generate_date("2022-01-01"), generate_date("2022-12-31")),
                (generate_date("2023-01-01"), generate_date("2023-08-31")),
            ],
        ),
        case(
            generate_date("2022-01-01"),
            generate_date("2024-08-31"),
            vec![
                (generate_date("2022-01-01"), generate_date("2022-12-31")),
                (generate_date("2023-01-01"), generate_date("2023-12-31")),
                (generate_date("2024-01-01"), generate_date("2024-08-31")),
            ],
        ),
    )]
    fn test_date_divider(
        since: Date<Utc>,
        until: Date<Utc>,
        expected: Vec<(Date<Utc>, Date<Utc>)>,
    ) {
        let actual = date_divider(since, until);
        assert_eq!(actual, expected);
    }
}
