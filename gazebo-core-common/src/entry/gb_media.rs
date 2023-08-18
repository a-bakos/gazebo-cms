use crate::{account::gb_account::AccountID, entry::entry_id::EntryID};

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GB_Media {
    pub id: EntryID,
    pub uploader: AccountID,
    pub attached_to: Vec<EntryID>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub alt_text: Option<String>,
}
