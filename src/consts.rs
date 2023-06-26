use crate::users::user::UserID;

/// App default
pub const DEFAULT_APP_NAME: &str = "Gazebo CMS";
pub const DEFAULT_APP_ADMIN_EMAIL: &str = "change_this@gazebocms.email";
pub const VERSION: &str = "0.0.331";
pub const SYSTEM_USER_ID: UserID = UserID(0);

/// Labels
pub const LABEL_NONE: &str = "none";
pub const LABEL_UNKNOWN: &str = "unknown";
pub const LABEL_YES: &str = "yes";
pub const LABEL_NO: &str = "no";

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

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MIN_USER_NAME_LENGTH: usize = 4;

#[allow(dead_code)]
pub const USER_ID_TEMPORARY_DEFAULT: u32 = 0;

pub const USER_ROLE_ADMIN: &str = "administrator";
pub const USER_ROLE_EDITOR: &str = "editor";
pub const USER_ROLE_CONTRIBUTOR: &str = "contributor";
pub const USER_ROLE_NOT_FOUND: &str = "not_found";

pub const ENTRY_TYPE_POST: &str = "post";
pub const ENTRY_TYPE_PAGE: &str = "page";
pub const ENTRY_TYPE_MEDIA: &str = "media";
pub const ENTRY_TYPE_UNKNOWN: &str = "unknown";

pub const ENTRY_ID_NO_PARENT: i32 = 0;
