/// Permalink defaults
pub const PERMALINK_MAX_ALLOWED_LENGTH: usize = 1024;
pub const DEFAULT_PERMALINK_SEPARATOR: &str = "-";
pub const DEFAULT_PERMALINK_LIMIT: usize = 60;
pub const DEFAULT_PERMALINK_STOP_WORDS: [&str; 4] = ["and", "the", "of", "a"];

pub const DEFAULT_PERMALINK_NOT_ALLOWED_CHARS: [&str; 9] =
    ["&", "#", "?", "%", "<", ">", "\"", "'", "/"];
