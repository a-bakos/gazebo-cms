use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Post,
    Page,
    Media,
    Unknown,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Post => write!(f, "{}", crate::consts::ENTRY_TYPE_POST),
            EntryType::Page => write!(f, "{}", crate::consts::ENTRY_TYPE_PAGE),
            EntryType::Media => write!(f, "{}", crate::consts::ENTRY_TYPE_MEDIA),
            EntryType::Unknown => write!(f, "{}", crate::consts::ENTRY_TYPE_UNKNOWN),
        }
    }
}

impl Into<String> for EntryType {
    fn into(self) -> String {
        match self {
            EntryType::Post => crate::consts::ENTRY_TYPE_POST.to_string(),
            EntryType::Page => crate::consts::ENTRY_TYPE_PAGE.to_string(),
            EntryType::Media => crate::consts::ENTRY_TYPE_MEDIA.to_string(),
            EntryType::Unknown => crate::consts::ENTRY_TYPE_UNKNOWN.to_string(),
        }
    }
}

pub fn get_entry_type_variant(entry_type: &str) -> EntryType {
    match entry_type {
        crate::consts::ENTRY_TYPE_POST => EntryType::Post,
        crate::consts::ENTRY_TYPE_PAGE => EntryType::Page,
        crate::consts::ENTRY_TYPE_MEDIA => EntryType::Media,
        _ => EntryType::Unknown,
    }
}
