use crate::account::gb_account::AccountID;
use crate::entry::entry_id::EntryID;
use crate::entry::gb_media::GB_Media;
use crate::entry::gb_post::GB_Post;
use crate::entry::status::{EntryStatus, MediaStatus};

enum GB_EntryDateVariant {
    Publish,
    Modified,
}

trait GB_Entry {
    fn get_id(&self) -> EntryID;

    fn get_author_id(&self) -> AccountID;

    fn get_date(&self, date_variant: GB_EntryDateVariant) -> String;

    fn get_slug(&self) -> String;

    fn get_status(&self) -> EntryStatus;

    fn get_title(&self) -> String;
}

impl GB_Entry for GB_Post {
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

impl GB_Entry for GB_Media {
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

// A generic example
fn get_id<T: GB_Entry>(entry: &T) -> EntryID {
    entry.get_id()
}

fn temp() {
    let entries: Vec<Box<dyn GB_Entry>> = vec![
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
