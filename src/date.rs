use chrono::prelude::*;

pub fn get_current_date() -> String {
    let date = OX_DateTime::new();
    date.formatted
}

struct OX_DateTime {
    utc: DateTime<Utc>,
    formatted: String,
}

impl OX_DateTime {
    pub fn new() -> Self {
        Self {
            utc: Utc::now(),
            formatted: format!("{}", utc.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}
