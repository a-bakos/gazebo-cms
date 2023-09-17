use crate::entry::entry_type::EntryType;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Post | Page entry "draft" status
pub const ENTRY_STATUS_CONTENT_DRAFT: &str = "draft";
/// Post | Page entry "publish" status
pub const ENTRY_STATUS_CONTENT_PUBLISH: &str = "publish";
/// Post | Page entry "private" status
pub const ENTRY_STATUS_CONTENT_PRIVATE: &str = "private";
/// Post | Page entry "trash" status
pub const ENTRY_STATUS_CONTENT_TRASH: &str = "trash";

/// Any entry "unknown" status
pub const ENTRY_STATUS_UNKNOWN: &str = "unknown";

/// Media entry "attached" status
pub const ENTRY_STATUS_MEDIA_ATTACHED: &str = "attached";
/// Media entry "unattached" status
pub const ENTRY_STATUS_MEDIA_UNATTACHED: &str = "unattached";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntryStatus {
    Post(ContentStatus),
    Media(MediaStatus),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentStatus {
    Draft,
    Publish,
    Private,
    Trash,
    Unknown,
    // Future
    // Pending
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaStatus {
    Attached,
    Unattached,
    Unknown,
}

pub fn get_entry_status_as_string(entry_status: EntryStatus) -> String {
    match entry_status {
        EntryStatus::Post(content_status) => match content_status {
            ContentStatus::Draft => ENTRY_STATUS_CONTENT_DRAFT.to_string(),
            ContentStatus::Publish => ENTRY_STATUS_CONTENT_PUBLISH.to_string(),
            ContentStatus::Private => ENTRY_STATUS_CONTENT_PRIVATE.to_string(),
            ContentStatus::Trash => ENTRY_STATUS_CONTENT_TRASH.to_string(),
            ContentStatus::Unknown => ENTRY_STATUS_UNKNOWN.to_string(),
        },
        EntryStatus::Media(media_status) => match media_status {
            MediaStatus::Attached => ENTRY_STATUS_MEDIA_ATTACHED.to_string(),
            MediaStatus::Unattached => ENTRY_STATUS_MEDIA_UNATTACHED.to_string(),
            MediaStatus::Unknown => ENTRY_STATUS_UNKNOWN.to_string(),
        },
        EntryStatus::Unknown => {
            // "unreachable_unknown".to_string()
            unreachable!()
        }
    }
}

impl std::fmt::Display for ContentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentStatus::Draft => write!(f, "{}", ENTRY_STATUS_CONTENT_DRAFT),
            ContentStatus::Publish => write!(f, "{}", ENTRY_STATUS_CONTENT_PUBLISH),
            ContentStatus::Private => write!(f, "{}", ENTRY_STATUS_CONTENT_PRIVATE),
            ContentStatus::Trash => write!(f, "{}", ENTRY_STATUS_CONTENT_TRASH),
            ContentStatus::Unknown => write!(f, "{}", ENTRY_STATUS_UNKNOWN),
        }
    }
}

impl From<ContentStatus> for String {
    fn from(val: ContentStatus) -> Self {
        match val {
            ContentStatus::Draft => ENTRY_STATUS_CONTENT_DRAFT.to_string(),
            ContentStatus::Publish => ENTRY_STATUS_CONTENT_PUBLISH.to_string(),
            ContentStatus::Private => ENTRY_STATUS_CONTENT_PRIVATE.to_string(),
            ContentStatus::Trash => ENTRY_STATUS_CONTENT_TRASH.to_string(),
            ContentStatus::Unknown => ENTRY_STATUS_UNKNOWN.to_string(),
        }
    }
}

impl From<String> for ContentStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            ENTRY_STATUS_CONTENT_DRAFT => ContentStatus::Draft,
            ENTRY_STATUS_CONTENT_PUBLISH => ContentStatus::Publish,
            ENTRY_STATUS_CONTENT_PRIVATE => ContentStatus::Private,
            ENTRY_STATUS_CONTENT_TRASH => ContentStatus::Trash,
            _ => ContentStatus::Unknown,
        }
    }
}

impl std::fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaStatus::Attached => write!(f, "{}", ENTRY_STATUS_MEDIA_ATTACHED),
            MediaStatus::Unattached => write!(f, "{}", ENTRY_STATUS_MEDIA_UNATTACHED),
            MediaStatus::Unknown => write!(f, "{}", ENTRY_STATUS_UNKNOWN),
        }
    }
}

impl From<MediaStatus> for String {
    fn from(val: MediaStatus) -> Self {
        match val {
            MediaStatus::Attached => ENTRY_STATUS_MEDIA_ATTACHED.to_string(),
            MediaStatus::Unattached => ENTRY_STATUS_MEDIA_UNATTACHED.to_string(),
            MediaStatus::Unknown => ENTRY_STATUS_UNKNOWN.to_string(),
        }
    }
}

pub fn get_entry_status_variant(
    entry_status_as_str: &str,
    the_entry_type: &EntryType,
) -> EntryStatus {
    match *the_entry_type {
        EntryType::Post => {
            let post_status = get_content_status_variant(entry_status_as_str);
            EntryStatus::Post(post_status)
        }
        EntryType::Media => {
            let media_status = get_media_status_variant(entry_status_as_str);
            EntryStatus::Media(media_status)
        }
        _ => EntryStatus::Unknown,
    }
}

pub fn get_content_status_variant(entry_status_as_str: &str) -> ContentStatus {
    match entry_status_as_str {
        ENTRY_STATUS_CONTENT_DRAFT => ContentStatus::Draft,
        ENTRY_STATUS_CONTENT_PUBLISH => ContentStatus::Publish,
        ENTRY_STATUS_CONTENT_PRIVATE => ContentStatus::Private,
        ENTRY_STATUS_CONTENT_TRASH => ContentStatus::Trash,
        _ => ContentStatus::Publish,
    }
}

pub fn get_media_status_variant(entry_status_as_str: &str) -> MediaStatus {
    match entry_status_as_str {
        ENTRY_STATUS_MEDIA_ATTACHED => MediaStatus::Attached,
        ENTRY_STATUS_MEDIA_UNATTACHED => MediaStatus::Unattached,
        _ => MediaStatus::Unknown,
    }
}
