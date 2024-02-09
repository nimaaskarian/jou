use chrono::{Local ,DateTime, FixedOffset};
use chrono::format::ParseError;
// const FORMAT: &str = "%Y-%m-%d.%H:%M:%S";
const FORMAT: &str = "%Y-%m-%d-%H-%M-%S";

#[inline]
pub fn parse(date_string: &String) -> Result<DateTime<FixedOffset>, ParseError> {
    DateTime::parse_from_str(date_string.as_str(), FORMAT)
}

#[inline]
pub fn current_string() -> String {
    format(Some(current()))
}

#[inline]
pub fn current() -> DateTime<Local> {
    Local::now()
}

#[inline]
pub fn format(input: Option<DateTime<Local>>) -> String {
    match input {
        Some(date)=> date.format(FORMAT).to_string(),
        None => String::new(),
    }
}
