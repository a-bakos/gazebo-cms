use crate::{entry::gb_post::EntryID, users::user::UserID};

struct GB_MediaItem {
    id: EntryID,
    uploader: UserID,
    attached_to: Vec<EntryID>,
    date_publish: String,
    date_modified: String,
    slug: String,
    title: String,
    description: Option<String>,
    alt_text: Option<String>,
}
