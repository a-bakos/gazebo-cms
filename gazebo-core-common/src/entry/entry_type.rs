use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Post,
    Page,
    Log,
    Media,
    Search,
    Unknown,
}
