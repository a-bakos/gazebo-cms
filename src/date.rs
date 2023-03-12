use chrono::prelude::*;
use std::fmt::Formatter;

pub fn get_current_date() -> String {
    let date: OX_DateTime = OX_DateTime::new();
    date.formatted
}

#[allow(non_camel_case_types)]
struct OX_DateTime {
    utc: DateTime<Utc>,
    formatted: String,
}

impl OX_DateTime {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            utc: now,
            formatted: format!("{}", now.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}

impl std::fmt::Display for OX_DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted)
    }
}
