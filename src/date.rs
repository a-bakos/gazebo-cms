use chrono::prelude::*;

pub fn get_current_date() -> String {
    let utc: DateTime<Utc> = Utc::now();
    let formatted = format!("{}", utc.format("%Y-%m-%d %H:%M:%S"));
    formatted
}
