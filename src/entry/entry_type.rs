use serde::{Deserialize, Serialize};

use crate::consts::{
    ENTRY_TYPE_LOG, ENTRY_TYPE_MEDIA, ENTRY_TYPE_PAGE, ENTRY_TYPE_POST, ENTRY_TYPE_SEARCH,
    ENTRY_TYPE_UNKNOWN,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Post,
    Page,
    Log,
    Media,
    Search,
    Unknown,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Post => write!(f, "{}", ENTRY_TYPE_POST),
            EntryType::Page => write!(f, "{}", ENTRY_TYPE_PAGE),
            EntryType::Log => write!(f, "{}", ENTRY_TYPE_LOG),
            EntryType::Media => write!(f, "{}", ENTRY_TYPE_MEDIA),
            EntryType::Search => write!(f, "{}", ENTRY_TYPE_SEARCH),
            EntryType::Unknown => write!(f, "{}", ENTRY_TYPE_UNKNOWN),
        }
    }
}

impl Into<String> for EntryType {
    fn into(self) -> String {
        match self {
            EntryType::Post => ENTRY_TYPE_POST.to_string(),
            EntryType::Page => ENTRY_TYPE_PAGE.to_string(),
            EntryType::Log => ENTRY_TYPE_LOG.to_string(),
            EntryType::Media => ENTRY_TYPE_MEDIA.to_string(),
            EntryType::Search => ENTRY_TYPE_SEARCH.to_string(),
            EntryType::Unknown => ENTRY_TYPE_UNKNOWN.to_string(),
        }
    }
}

pub fn get_entry_type_variant(entry_type: &str) -> EntryType {
    match entry_type {
        ENTRY_TYPE_POST => EntryType::Post,
        ENTRY_TYPE_PAGE => EntryType::Page,
        ENTRY_TYPE_LOG => EntryType::Log,
        ENTRY_TYPE_MEDIA => EntryType::Media,
        ENTRY_TYPE_SEARCH => EntryType::Search,
        _ => EntryType::Unknown,
    }
}
