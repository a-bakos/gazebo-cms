use crate::account::gb_account::AccountID;
use crate::entry::entry_id::EntryID;
use crate::entry::gb_media::GB_Media;
use crate::entry::gb_post::GB_Post;
use crate::entry::status::{EntryStatus, MediaStatus};

use serde::{Deserialize, Serialize};

pub enum GB_EntryDateVariant {
    Publish,
    Modified,
}

pub trait GB_EntryCommon {
    fn get_id(&self) -> EntryID;

    fn get_author_id(&self) -> AccountID;

    // todo
    // fn get_author(&self) -> String;

    fn get_date(&self, date_variant: GB_EntryDateVariant) -> String;

    fn get_slug(&self) -> String;

    fn get_status(&self) -> EntryStatus;

    fn get_title(&self) -> String;
}

// A generic example
fn get_id<T: GB_EntryCommon>(entry: &T) -> EntryID {
    entry.get_id()
}

// temp example for later - this way, we can keep different entry types in a collection
// eg. entry search for matching items such as same "editor" or same publish date
fn temp() {
    let entries: Vec<Box<dyn GB_EntryCommon>> = vec![
        Box::new(GB_Post {
            id: Default::default(),
            id_author: Default::default(),
            id_parent: None,
            date_publish: "".to_string(),
            date_modified: "".to_string(),
            slug: None,
            status: EntryStatus::Unknown,
            title: None,
            excerpt: None,
            content: None,
            password: None,
        }),
        Box::new(GB_Media {
            id: Default::default(),
            uploader: Default::default(),
            attached_to: vec![],
            date_publish: "".to_string(),
            date_modified: "".to_string(),
            slug: "".to_string(),
            title: "".to_string(),
            description: None,
            alt_text: None,
        }),
    ];

    for entry in entries.iter() {
        println!("{}", entry.get_id());
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GB_EntryInsertRequest {
    pub author_id: i32,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: String,
    pub excerpt: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GB_EntryInsertResponse {
    pub http_status_code: u32,
    pub entry_id: EntryID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GB_EntryUpdateRequest {
    pub author_id: AccountID,
    pub entry_id: EntryID,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: String,
    pub excerpt: Option<String>,
    pub password: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GB_EntryUpdateResponse {
    pub http_status_code: u32,
    pub entry_id: EntryID,
}
