use crate::Result;
use chrono::{
    format::{parse as chrono_parse, Parsed, StrftimeItems},
    DateTime, FixedOffset, NaiveDateTime, NaiveTime, Utc,
};

const NOW: &str = "now";
const TIME_HOUR_24: &str = "%H";
const TIME_HOUR_12: &str = "%I%p";
const TIME_HOUR_MIN_24: &str = "%H:%M";
const TIME_HOUR_MIN_12: &str = "%I:%M%p";

const DATE_TIME_1: &str = "%d.%m.%Y %H:%M:%S";
const DATE_TIME_2: &str = "%Y-%m-%d %H:%M:%S";

pub fn parse_into_date(str_date: &str, offset: FixedOffset) -> Result<DateTime<Utc>> {
    let low_str_date = str_date.trim().to_lowercase();

    if str_date.to_lowercase() == NOW {
        return Ok(Utc::now());
    }

    //parse hour fragments

    //match NaiveTime::parse_from_str(&low_str_date, TIME_HOUR_24) {
    match parsed.to_naive_time() {
        Ok(v) => {
            let nd = NaiveDateTime::new(Utc::now().with_timezone(&offset).date().naive_local(), v);
            return Ok(DateTime::<FixedOffset>::from_utc(nd, offset).with_timezone(&Utc));
        }
        Err(e) => {
            println!(
                "Unable to parse {} with pattern {}: {:?}",
                low_str_date, TIME_HOUR_24, e
            );
            log::debug!("Unable to parse time with pattern {}", TIME_HOUR_24)
        }
    }

    //DateTime::parse_from_str(low_str_date, MIN_TIME);
    //DateTime::parse_from_str(low_str_date, TIME);

    //DateTime::parse_from_str(low_str_date, DATE_TIME_1);
    //DateTime::parse_from_str(low_str_date, DATE_TIME_2);

    //DateTime::parse_from_rfc2822(low_str_date);

    //DateTime::parse_from_rfc3339(low_str_date);

    return Err(crate::error::Error::InvalidCommandError);
}

fn try_to_parse_hour_fragment(
    str_date: &str,
    offset: FixedOffset,
    pattern: &str,
) -> Result<DateTime<Utc>> {
    let mut parsed = Parsed::new();
    chrono_parse(&mut parsed, &str_date, StrftimeItems::new(TIME_HOUR_24))
        .map_err(|_| crate::error::Error::InvalidCommandError)?;
    parsed.set_minute(0)?;
    parsed.set_second(0)?;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_parse_now() {
        assert_eq_close_to(
            parse_into_date("NOW", FixedOffset::east(0)),
            Ok(Utc::now()),
            20,
        );

        assert_eq_close_to(
            parse_into_date("now", FixedOffset::east(0)),
            Ok(Utc::now()),
            20,
        );
    }

    #[test]
    fn test_parse_hour_24() {
        let now = Utc::now();
        assert_eq_close_to(
            parse_into_date("15", FixedOffset::east(0)),
            Ok(Utc::now()), //.and_hms(15, 0, 0)),
            20,
        );
    }

    fn assert_eq_close_to(
        actual: Result<DateTime<Utc>>,
        expected: Result<DateTime<Utc>>,
        diff_in_ms: i64,
    ) {
        match (actual, expected) {
            (Ok(a), Ok(e)) => {
                let actual_ms = a.timestamp_millis();
                let expected_ms = e.timestamp_millis();
                if (expected_ms - diff_in_ms) > actual_ms || actual_ms > (expected_ms + diff_in_ms)
                {
                    assert!(
                        false,
                        "Expected value {} wasn't near actual value {} (allowed delta in ms {})",
                        a, e, diff_in_ms
                    );
                }
            }
            (Err(a), Err(e)) => assert_eq!(a, e),
            (Ok(a), Err(e)) => assert!(
                false,
                "Actual value was ok({}) but expected value was an error({})",
                a, e
            ),
            (Err(a), Ok(e)) => assert!(
                false,
                "Actual value was an error({}) but expected value was ok({})",
                a, e
            ),
        }
    }
}
