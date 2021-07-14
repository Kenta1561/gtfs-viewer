use chrono::{Duration, NaiveDate, ParseResult};
use regex::Regex;

const DATE_FORMAT: &str = "%Y%m%d";

pub fn str_to_date(val: String) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(&val, DATE_FORMAT)
}

pub fn str_to_dur(regex: &Regex, val: String) -> ParseResult<Duration> {
    let caps = regex.captures(&val).unwrap();
    Ok(Duration::hours(caps["hours"].parse::<i64>().unwrap()) +
        Duration::minutes(caps["minutes"].parse::<i64>().unwrap()) +
        Duration::seconds(caps["seconds"].parse::<i64>().unwrap())
    )
}
