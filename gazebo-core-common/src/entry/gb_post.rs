use crate::account::consts::NEW_ACCOUNT_TEMP_ID;
use crate::account::gb_account::AccountID;
use crate::datetime::functions::get_current_date;
use crate::entry::consts::NEW_ENTRY_TEMP_ID;
use crate::entry::entry_id::{get_entry_parent_id, EntryID};
use crate::entry::status::{ContentStatus, EntryStatus};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GB_Post {
    pub id: EntryID,
    pub id_author: AccountID,
    pub id_parent: Option<EntryID>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: Option<String>,
    pub status: EntryStatus,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub password: Option<String>,
}

impl GB_Post {
    pub fn new() -> Self {
        Self {
            id: NEW_ENTRY_TEMP_ID,
            id_author: NEW_ACCOUNT_TEMP_ID,
            id_parent: None,
            date_publish: String::new(),
            date_modified: String::new(),
            slug: None,
            status: EntryStatus::Unknown,
            title: None,
            excerpt: None,
            content: None,
            password: None,
        }
    }
}
