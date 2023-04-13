pub const VERSION: &str = "0.0.2";

// Permalink defaults
#[allow(dead_code)]
pub const PERMALINK_MAX_ALLOWED_LENGTH: usize = 1024;
pub const DEFAULT_PERMALINK_SEPARATOR: &'static str = "-";
pub const DEFAULT_PERMALINK_LIMIT: usize = 60;
pub const DEFAULT_PERMALINK_STOP_WORDS: [&'static str; 4] = ["and", "the", "of", "a"];
pub const DEFAULT_PERMALINK_NOT_ALLOWED_CHARS: [&'static str; 9] =
    ["&", "#", "?", "%", "<", ">", "\"", "'", "/"];

pub const ID_START_VALUE: u32 = 0;

// Mock DB file
pub const FILE_PATH_POSTS: &'static str = "mock_db_posts.csv";
pub const FILE_PATH_USERS: &'static str = "mock_db_users.csv";

pub const USER_ID_TEMPORARY_DEFAULT: u32 = 0;

pub const MIN_PASSWORD_LENGTH: usize = 8;

pub const USER_ROLE_ADMIN: &'static str = "admin";
pub const USER_ROLE_EDITOR: &'static str = "editor";
pub const USER_ROLE_CONTRIBUTOR: &'static str = "contributor";
