use crate::account::gb_account::AccountID;
use crate::datetime::functions::get_current_date;
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

impl GB_Post {}
