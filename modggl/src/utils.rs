use chrono::{Date, DateTime, Duration, NaiveDate, Utc};

pub fn datetime_generator(value: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from(DateTime::parse_from_rfc3339(value).unwrap())
}

pub fn date_generator(value: &str) -> Date<Utc> {
    Date::<Utc>::from_utc(NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap(), Utc)
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
            date_generator("2022-01-01"),
            date_generator("2022-08-31"),
            vec![(date_generator("2022-01-01"), date_generator("2022-08-31"))],
        ),
        case(
            date_generator("2022-01-01"),
            date_generator("2022-12-31"),
            vec![(date_generator("2022-01-01"), date_generator("2022-12-31"))],
        ),
        case(
            date_generator("2022-01-01"),
            date_generator("2023-08-31"),
            vec![
                (date_generator("2022-01-01"), date_generator("2022-12-31")),
                (date_generator("2023-01-01"), date_generator("2023-08-31")),
            ],
        ),
        case(
            date_generator("2022-01-01"),
            date_generator("2024-08-31"),
            vec![
                (date_generator("2022-01-01"), date_generator("2022-12-31")),
                (date_generator("2023-01-01"), date_generator("2023-12-31")),
                (date_generator("2024-01-01"), date_generator("2024-08-31")),
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
