use crate::{
    database::columns::{
        COL_INDEX_LOG_ACCOUNT_EMAIL, COL_INDEX_LOG_ACCOUNT_ID, COL_INDEX_LOG_EVENT_CODE,
        COL_INDEX_LOG_EVENT_DATE, COL_INDEX_LOG_ID, COL_INDEX_LOG_SUBJECT_DESCRIPTION,
        COL_INDEX_LOG_SUBJECT_FROM, COL_INDEX_LOG_SUBJECT_ID, COL_INDEX_LOG_SUBJECT_TITLE,
        COL_INDEX_LOG_SUBJECT_TO, COL_INDEX_LOG_SUBJECT_URL,
    },
    traits::RowTransformer,
};
use chrono::NaiveDateTime;
use gazebo_core_common::{
    account::gb_account::AccountID,
    entry::gb_log::{GB_EventCode, GB_Log},
};
use sqlx::{postgres::PgRow, Row};

impl RowTransformer<PgRow> for GB_Log {
    type Output = GB_Log;

    fn transform(row: &PgRow) -> Self::Output {
        // Underscores' meaning here:
        // we don't need to specify a default/fallback value because the cell will never be empty

        let event_id = row.get::<i32, _>(COL_INDEX_LOG_ID);
        let account_id = row.get::<i32, _>(COL_INDEX_LOG_ACCOUNT_ID) as u32;
        let account_email = row.get(COL_INDEX_LOG_ACCOUNT_EMAIL);
        let event_code = row.get::<i32, _>(COL_INDEX_LOG_EVENT_CODE) as u32;
        let event_date: NaiveDateTime = row.get::<NaiveDateTime, _>(COL_INDEX_LOG_EVENT_DATE);
        let event_date = event_date.to_string();
        let subject_id = row.get::<i32, _>(COL_INDEX_LOG_SUBJECT_ID) as u32;
        let subject_url = row.get(COL_INDEX_LOG_SUBJECT_URL);
        let subject_title = row.get(COL_INDEX_LOG_SUBJECT_TITLE);
        let subject_description = row.get(COL_INDEX_LOG_SUBJECT_DESCRIPTION);
        let subject_from = row.get(COL_INDEX_LOG_SUBJECT_FROM);
        let subject_to = row.get(COL_INDEX_LOG_SUBJECT_TO);

        Self {
            account_id: AccountID(account_id),
            account_email,
            event_code,
            event_date,
            subject_id,
            subject_url,
            subject_title,
            subject_description,
            subject_from,
            subject_to,
        }
    }
}
