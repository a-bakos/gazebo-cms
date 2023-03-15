use chrono::prelude::*;
use std::fmt::Formatter;

#[allow(non_camel_case_types)]
pub(crate) struct OX_DateTime {
    utc: DateTime<Utc>,
    pub formatted: String,
}

impl OX_DateTime {
    pub(crate) fn new() -> Self {
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
