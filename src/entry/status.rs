use crate::entry::entry_type::EntryType;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

pub const ENTRY_STATUS_POST_DRAFT: &str = "draft";
pub const ENTRY_STATUS_POST_PUBLISH: &str = "publish";
pub const ENTRY_STATUS_POST_PRIVATE: &str = "private";
pub const ENTRY_STATUS_POST_TRASH: &str = "trash";
pub const ENTRY_STATUS_MEDIA_ATTACHED: &str = "attached";
pub const ENTRY_STATUS_MEDIA_UNATTACHED: &str = "unattached";

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
            PostStatus::Draft => write!(f, "draft"),
            PostStatus::Publish => write!(f, "publish"),
            PostStatus::Private => write!(f, "private"),
            PostStatus::Trash => write!(f, "trash"),
            PostStatus::Unknown => write!(f, "unknown"),
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
            MediaStatus::Attached => write!(f, "attached"),
            MediaStatus::Unattached => write!(f, "unattached"),
            MediaStatus::Unknown => write!(f, "unknown"),
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
