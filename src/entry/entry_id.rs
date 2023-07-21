use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

// New type patterns for IDs
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct EntryID(pub u32);

impl std::fmt::Display for EntryID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for EntryID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[allow(dead_code)]
pub fn get_entry_parent_id() -> Option<EntryID> {
    // if parent
    // Some(EntryID(10))
    // else
    None
}
