use std::fmt::Formatter;

#[derive(Debug)]
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
            EntryType::Post => write!(f, "posts"),
            EntryType::Page => write!(f, "page"),
            EntryType::Media => write!(f, "media"),
        }
    }
}
