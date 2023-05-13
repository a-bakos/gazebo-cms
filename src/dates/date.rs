use chrono::prelude::*;
use std::fmt::Formatter;

#[allow(non_camel_case_types)]
pub(crate) struct GB_DateTime {
    #[allow(dead_code)]
    utc: DateTime<Utc>,
    pub formatted: String,
}

impl GB_DateTime {
    pub(crate) fn new() -> Self {
        let now = Utc::now();
        Self {
            utc: now,
            formatted: format!("{}", now.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}

impl std::fmt::Display for GB_DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted)
    }
}

// Todo
// define( 'MINUTE_IN_SECONDS', 60 );
// define( 'HOUR_IN_SECONDS', 60 * MINUTE_IN_SECONDS );
// define( 'DAY_IN_SECONDS', 24 * HOUR_IN_SECONDS );
// define( 'WEEK_IN_SECONDS', 7 * DAY_IN_SECONDS );
