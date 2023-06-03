use crate::entry::entry_type::EntryType;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Post entry "draft" status
pub const ENTRY_STATUS_POST_DRAFT: &str = "draft";
/// Post entry "publish" status
pub const ENTRY_STATUS_POST_PUBLISH: &str = "publish";
/// Post entry "private" status
pub const ENTRY_STATUS_POST_PRIVATE: &str = "private";
/// Post entry "trash" status
pub const ENTRY_STATUS_POST_TRASH: &str = "trash";
/// Post entry "unknown" status
pub const ENTRY_STATUS_POST_UNKNOWN: &str = "unknown";

/// Media entry "attached" status
pub const ENTRY_STATUS_MEDIA_ATTACHED: &str = "attached";
/// Media entry "unattached" status
pub const ENTRY_STATUS_MEDIA_UNATTACHED: &str = "unattached";
/// Media entry "unknown" status
pub const ENTRY_STATUS_MEDIA_UNKNOWN: &str = "unknown";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryStatus {
    PostStatus(PostStatus),
    //PageStatus(PageStatus),
    MediaStatus(MediaStatus),
    UnknownStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostStatus {
    Draft,
    Publish,
    Private,
    Trash,
    Unknown,
}
impl std::fmt::Display for PostStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PostStatus::Draft => write!(f, "{}", ENTRY_STATUS_POST_DRAFT),
            PostStatus::Publish => write!(f, "{}", ENTRY_STATUS_POST_PUBLISH),
            PostStatus::Private => write!(f, "{}", ENTRY_STATUS_POST_PRIVATE),
            PostStatus::Trash => write!(f, "{}", ENTRY_STATUS_POST_TRASH),
            PostStatus::Unknown => write!(f, "{}", ENTRY_STATUS_POST_UNKNOWN),
        }
    }
}

impl Into<String> for PostStatus {
    fn into(self) -> String {
        match self {
            PostStatus::Draft => ENTRY_STATUS_POST_DRAFT.to_string(),
            PostStatus::Publish => ENTRY_STATUS_POST_PUBLISH.to_string(),
            PostStatus::Private => ENTRY_STATUS_POST_PRIVATE.to_string(),
            PostStatus::Trash => ENTRY_STATUS_POST_TRASH.to_string(),
            PostStatus::Unknown => ENTRY_STATUS_POST_UNKNOWN.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaStatus {
    Attached,
    Unattached,
    Unknown,
}
impl std::fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaStatus::Attached => write!(f, "{}", ENTRY_STATUS_MEDIA_ATTACHED),
            MediaStatus::Unattached => write!(f, "{}", ENTRY_STATUS_MEDIA_UNATTACHED),
            MediaStatus::Unknown => write!(f, "{}", ENTRY_STATUS_MEDIA_UNKNOWN),
        }
    }
}

impl Into<String> for MediaStatus {
    fn into(self) -> String {
        match self {
            MediaStatus::Attached => ENTRY_STATUS_MEDIA_ATTACHED.to_string(),
            MediaStatus::Unattached => ENTRY_STATUS_MEDIA_UNATTACHED.to_string(),
            MediaStatus::Unknown => ENTRY_STATUS_MEDIA_UNKNOWN.to_string(),
        }
    }
}

pub fn get_entry_status_variant(
    entry_status_as_str: &str,
    the_entry_type: &EntryType,
) -> EntryStatus {
    match *the_entry_type {
        EntryType::Post => {
            let post_status = get_post_status_variant(entry_status_as_str);
            EntryStatus::PostStatus(post_status)
        }
        // EntryType::Page => {
        //     let page_status: PostStatus = get_page_status_variant();
        //     EntryStatus::PageStatus(page_status)
        // }
        EntryType::Media => {
            let media_status = get_media_status_variant(entry_status_as_str);
            EntryStatus::MediaStatus(media_status)
        }
        _ => EntryStatus::UnknownStatus,
    }
}

pub fn get_post_status_variant(entry_status_as_str: &str) -> PostStatus {
    return match entry_status_as_str {
        ENTRY_STATUS_POST_DRAFT => PostStatus::Draft,
        ENTRY_STATUS_POST_PUBLISH => PostStatus::Publish,
        ENTRY_STATUS_POST_PRIVATE => PostStatus::Private,
        ENTRY_STATUS_POST_TRASH => PostStatus::Trash,
        _ => PostStatus::Publish,
    };
}

pub fn get_media_status_variant(entry_status_as_str: &str) -> MediaStatus {
    return match entry_status_as_str {
        ENTRY_STATUS_MEDIA_ATTACHED => MediaStatus::Attached,
        ENTRY_STATUS_MEDIA_UNATTACHED => MediaStatus::Unattached,
        _ => MediaStatus::Unknown,
    };
}
