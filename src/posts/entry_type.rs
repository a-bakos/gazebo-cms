use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Post,
    #[allow(dead_code)]
    Page,
    #[allow(dead_code)]
    Media,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Post => write!(f, "{}", crate::consts::ENTRY_TYPE_POST),
            EntryType::Page => write!(f, "{}", crate::consts::ENTRY_TYPE_PAGE),
            EntryType::Media => write!(f, "{}", crate::consts::ENTRY_TYPE_MEDIA),
        }
    }
}

pub fn get_entry_type_variant(entry_type: &str) -> EntryType {
    match entry_type {
        crate::consts::ENTRY_TYPE_POST => EntryType::Post,
        crate::consts::ENTRY_TYPE_PAGE => EntryType::Page,
        crate::consts::ENTRY_TYPE_MEDIA => EntryType::Media,
        _ => panic!("Unknown entry type"),
    }
}
