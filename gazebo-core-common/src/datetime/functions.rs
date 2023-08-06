use crate::datetime::date::GB_DateTime;

#[allow(dead_code)]
pub fn get_current_date() -> String {
    let date = GB_DateTime::new();
    date.formatted
}
