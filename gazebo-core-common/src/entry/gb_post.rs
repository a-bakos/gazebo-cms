use crate::{
    account::gb_account::AccountID,
    datetime::get_current_date,
    entry::{
        entry_id::{get_entry_parent_id, EntryID},
        gb_entry::{GB_EntryCommon, GB_EntryDateVariant},
        status::{ContentStatus, EntryStatus},
    },
};
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

impl GB_EntryCommon for GB_Post {
    fn get_id(&self) -> EntryID {
        self.id
    }

    fn get_author_id(&self) -> AccountID {
        self.id_author.clone()
    }

    fn get_date(&self, date_variant: GB_EntryDateVariant) -> String {
        match date_variant {
            GB_EntryDateVariant::Publish => self.date_publish.clone(),
            GB_EntryDateVariant::Modified => self.date_modified.clone(),
        }
    }

    fn get_slug(&self) -> String {
        if self.slug.is_some() {
            return self.slug.clone().unwrap();
        }
        "".to_string()
    }

    fn get_status(&self) -> EntryStatus {
        self.status.clone()
    }

    fn get_title(&self) -> String {
        if self.title.is_some() {
            return self.title.clone().unwrap();
        }
        "".to_string()
    }
}
