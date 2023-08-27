use chrono::{prelude::*, Duration};
use std::fmt::Formatter;

#[allow(dead_code)]
pub const MINUTE_IN_SECONDS: u32 = 60;
#[allow(dead_code)]
pub const HOUR_IN_SECONDS: u32 = 3600;
#[allow(dead_code)]
pub const DAY_IN_SECONDS: u32 = 86400;
#[allow(dead_code)]
pub const WEEK_IN_SECONDS: u32 = 604800;

pub enum GB_DateTime_Variant {
    Now,
    NextDaySameTime,
}

#[allow(non_camel_case_types)]
pub struct GB_DateTime {
    pub utc: DateTime<Utc>,
    pub timestamp: i64,
}

impl GB_DateTime {
    pub fn new(when: GB_DateTime_Variant) -> Self {
        let modifier = match when {
            GB_DateTime_Variant::Now => Utc::now(),
            GB_DateTime_Variant::NextDaySameTime => Utc::now() + Duration::days(1),
        };

        Self {
            utc: modifier,
            timestamp: modifier.timestamp(),
        }
    }
}

impl Into<String> for GB_DateTime {
    fn into(self) -> String {
        format!("{}", self.utc.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl std::fmt::Display for GB_DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.utc.to_string())
    }
}

pub fn get_current_date() -> String {
    let date = GB_DateTime::new(GB_DateTime_Variant::Now);
    date.to_string()
}
