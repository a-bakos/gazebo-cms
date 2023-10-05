use crate::database::columns::{
    COL_INDEX_LOG_ACCOUNT_EMAIL, COL_INDEX_LOG_ACCOUNT_ID, COL_INDEX_LOG_EVENT_CODE,
    COL_INDEX_LOG_EVENT_DATE, COL_INDEX_LOG_ID, COL_INDEX_LOG_SUBJECT_DESCRIPTION,
    COL_INDEX_LOG_SUBJECT_FROM, COL_INDEX_LOG_SUBJECT_ID, COL_INDEX_LOG_SUBJECT_TITLE,
    COL_INDEX_LOG_SUBJECT_TO, COL_INDEX_LOG_SUBJECT_URL,
};
use crate::traits::RowTransformer;
use chrono::{NaiveDate, NaiveDateTime};
use gazebo_core_common::account::gb_account::AccountID;
use gazebo_core_common::entry::gb_log::{GB_EventCode, GB_Log};
use sqlx::postgres::PgRow;
use sqlx::Row;

impl RowTransformer<PgRow> for GB_Log {
    type Output = GB_Log;

    fn transform(row: &PgRow) -> Self::Output {
        // Underscores' meaning here:
        // we don't need to specify a default/fallback value because the cell will never be empty

        // IDs
        let event_id = row.get::<i32, _>(COL_INDEX_LOG_ID);
        let account_id = row.get::<i32, _>(COL_INDEX_LOG_ACCOUNT_ID) as u32;
        let account_email = row.get(COL_INDEX_LOG_ACCOUNT_EMAIL);

        let event_code = row.get::<i32, _>(COL_INDEX_LOG_EVENT_CODE) as u32; // GB_EventCode,
        let event_code = GB_EventCode::from_value(event_code);

        // Date
        let event_date: NaiveDateTime = row.get::<NaiveDateTime, _>(COL_INDEX_LOG_EVENT_DATE);
        let event_date = event_date.to_string();

        let subject_id = row.get::<i32, _>(COL_INDEX_LOG_SUBJECT_ID) as u32; // u32,
        let subject_url = row.get(COL_INDEX_LOG_SUBJECT_URL); // Option<String>,
        let subject_title = row.get(COL_INDEX_LOG_SUBJECT_TITLE); // Option<String>,
        let subject_description = row.get(COL_INDEX_LOG_SUBJECT_DESCRIPTION); // Option<String>,
        let subject_from = row.get(COL_INDEX_LOG_SUBJECT_FROM); // Option<String>,
        let subject_to = row.get(COL_INDEX_LOG_SUBJECT_TO); // Option<String>,

        // todo here // placeholder transformer
        /*let parent_id = row
            .try_get(COL_INDEX_POST_PARENT)
            .ok()
            .unwrap_or(consts::ENTRY_ID_NO_PARENT) as u32;
        */
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
