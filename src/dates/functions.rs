use crate::dates::date::OX_DateTime;

pub fn get_current_date() -> String {
    let date: OX_DateTime = OX_DateTime::new();
    date.formatted
}
