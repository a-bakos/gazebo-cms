use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryStatus {
    Post(ContentStatus),
    Media(MediaStatus),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentStatus {
    Draft,
    Publish,
    Private,
    Trash,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaStatus {
    Attached,
    Unattached,
    Unknown,
}
