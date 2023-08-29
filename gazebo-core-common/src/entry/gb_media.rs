use crate::{
    account::gb_account::AccountID,
    entry::{
        entry_id::EntryID,
        gb_entry::{GB_EntryCommon, GB_EntryDateVariant},
        status::{EntryStatus, MediaStatus},
    },
};

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

impl GB_EntryCommon for GB_Media {
    fn get_id(&self) -> EntryID {
        self.id
    }

    fn get_author_id(&self) -> AccountID {
        self.uploader.clone()
    }

    fn get_date(&self, date_variant: GB_EntryDateVariant) -> String {
        match date_variant {
            GB_EntryDateVariant::Publish => self.date_publish.clone(),
            GB_EntryDateVariant::Modified => self.date_modified.clone(),
        }
    }

    fn get_slug(&self) -> String {
        self.slug.clone()
    }

    fn get_status(&self) -> EntryStatus {
        if self.attached_to.is_empty() {
            return EntryStatus::Media(MediaStatus::Unattached);
        }
        EntryStatus::Media(MediaStatus::Attached)
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }
}
