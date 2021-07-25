use crate::{error::Error::ParseChronoError, Result};
use chrono::{
    format::{parse as chrono_parse, Parsed, StrftimeItems},
    DateTime, Datelike, FixedOffset, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc,
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

    if low_str_date == NOW {
        return Ok(Utc::now());
    }

    if let Ok(dt) = try_to_parse_hour_fragment(&low_str_date, offset, TIME_HOUR_24) {
        return Ok(dt);
    }

    if let Ok(dt) = try_to_parse_hour_fragment(&low_str_date, offset, TIME_HOUR_12) {
        return Ok(dt);
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
    chrono_parse(&mut parsed, &str_date, StrftimeItems::new(pattern)).map_err(ParseChronoError)?;
    parsed.set_minute(0).map_err(ParseChronoError)?;
    parsed.set_second(0).map_err(ParseChronoError)?;

    match parsed.to_naive_time() {
        Ok(v) => {
            let now = Utc::now().with_timezone(&offset);

            let date = offset.ymd(now.year(), now.month(), now.day()).and_hms(
                v.hour(),
                v.minute(),
                v.second(),
            );
            Ok(date.with_timezone(&Utc))
        }
        Err(e) => {
            println!(
                "Unable to parse {} with pattern {}: {:?}",
                str_date, pattern, e
            );
            log::debug!(
                "Unable to parse time date `{}` with pattern `{}`",
                str_date,
                pattern
            );
            Err(ParseChronoError(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use chrono::{Datelike, TimeZone, Utc};

    const HOUR_IN_SEC: i32 = 3_600;

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
            Ok(Utc
                .ymd(now.year(), now.month(), now.day())
                .and_hms(15, 0, 0)),
            20,
        );

        assert_eq_close_to(
            parse_into_date("17", FixedOffset::east(2 * HOUR_IN_SEC)),
            Ok(Utc
                .ymd(now.year(), now.month(), now.day())
                .and_hms(15, 0, 0)),
            20,
        );
    }

    #[test]
    fn test_parse_hour_12() {
        let now = Utc::now();
        assert_eq_close_to(
            parse_into_date("6am", FixedOffset::east(0)),
            Ok(Utc.ymd(now.year(), now.month(), now.day()).and_hms(6, 0, 0)),
            20,
        );

        assert_eq_close_to(
            parse_into_date("6PM", FixedOffset::east(0)),
            Ok(Utc
                .ymd(now.year(), now.month(), now.day())
                .and_hms(18, 0, 0)),
            20,
        );

        assert_eq_close_to(
            parse_into_date("6pm", FixedOffset::east(2 * HOUR_IN_SEC)),
            Ok(Utc
                .ymd(now.year(), now.month(), now.day())
                .and_hms(16, 0, 0)),
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
