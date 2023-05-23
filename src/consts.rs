use crate::users::user::UserID;

/// App default
pub const DEFAULT_APP_NAME: &str = "Gazebo CMS";
pub const DEFAULT_APP_ADMIN_EMAIL: &str = "change_this@gazebocms.email";
pub const VERSION: &str = "0.0.169";
pub const SYSTEM_USER_ID: UserID = UserID(0);

/// Permalink defaults
#[allow(dead_code)]
pub const PERMALINK_MAX_ALLOWED_LENGTH: usize = 1024;
pub const DEFAULT_PERMALINK_SEPARATOR: &str = "-";
pub const DEFAULT_PERMALINK_LIMIT: usize = 60;
pub const DEFAULT_PERMALINK_STOP_WORDS: [&str; 4] = ["and", "the", "of", "a"];
pub const DEFAULT_PERMALINK_NOT_ALLOWED_CHARS: [&str; 9] =
    ["&", "#", "?", "%", "<", ">", "\"", "'", "/"];

pub const ID_START_VALUE: u32 = 0;
pub const POST_UNTITLED_DEFAULT: &str = "Untitled Gazebo Post";

// Mock DB (table) files
pub const FILE_PATH_POSTS: &str = "mock_db_posts.csv";
pub const FILE_PATH_USERS: &str = "mock_db_users.csv";

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MIN_USER_NAME_LENGTH: usize = 4;

pub const USER_ID_TEMPORARY_DEFAULT: u32 = 0;

pub const USER_ROLE_ADMIN: &str = "administrator";
pub const USER_ROLE_EDITOR: &str = "editor";
pub const USER_ROLE_CONTRIBUTOR: &str = "contributor";
pub const USER_ROLE_NOT_FOUND: &str = "not_found";

pub const ENTRY_TYPE_POST: &str = "post";
pub const ENTRY_TYPE_PAGE: &str = "page";
pub const ENTRY_TYPE_MEDIA: &str = "media";
