use crate::users::user::UserID;

/// App default
pub const DEFAULT_APP_NAME: &str = "Gazebo CMS";
pub const DEFAULT_APP_ADMIN_EMAIL: &str = "change_this@gazebocms.email";
pub const VERSION: &str = "0.0.402";
#[allow(dead_code)]
pub const SYSTEM_USER_ID: UserID = UserID(0);

/// Labels
pub const LABEL_NONE: &str = "none";
#[allow(dead_code)]
pub const LABEL_UNKNOWN: &str = "unknown";
#[allow(dead_code)]
pub const LABEL_YES: &str = "yes";
#[allow(dead_code)]
pub const LABEL_NO: &str = "no";

/// Messages
#[allow(dead_code)]
pub const MSG_LOGIN_SUCCESS: &str = "Login successful";

/// Permalink defaults
#[allow(dead_code)]
pub const PERMALINK_MAX_ALLOWED_LENGTH: usize = 1024;
#[allow(dead_code)]
pub const DEFAULT_PERMALINK_SEPARATOR: &str = "-";
#[allow(dead_code)]
pub const DEFAULT_PERMALINK_LIMIT: usize = 60;
#[allow(dead_code)]
pub const DEFAULT_PERMALINK_STOP_WORDS: [&str; 4] = ["and", "the", "of", "a"];
#[allow(dead_code)]
pub const DEFAULT_PERMALINK_NOT_ALLOWED_CHARS: [&str; 9] =
    ["&", "#", "?", "%", "<", ">", "\"", "'", "/"];

#[allow(dead_code)]
pub const POST_UNTITLED_DEFAULT_TITLE: &str = "Untitled Gazebo Post";
#[allow(dead_code)]
pub const POST_UNTITLED_DEFAULT_PERMALINK: &str = "untitled-gazebo-post";
#[allow(dead_code)]
pub const PAGE_UNTITLED_DEFAULT_TITLE: &str = "Untitled Gazebo Page";
#[allow(dead_code)]
pub const PAGE_UNTITLED_DEFAULT_PERMALINK: &str = "untitled-gazebo-page";

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MIN_USER_NAME_LENGTH: usize = 4;

#[allow(dead_code)]
pub const USER_ID_TEMPORARY_DEFAULT: u32 = 0;

pub const USER_ROLE_ADMIN: &str = "administrator";
pub const USER_ROLE_EDITOR: &str = "editor";
pub const USER_ROLE_CONTRIBUTOR: &str = "contributor";
pub const USER_ROLE_NOT_FOUND: &str = "not_found";

/// Entry type for blog-like posts
pub const ENTRY_TYPE_POST: &str = "post";
/// Entry type for top-level pages
pub const ENTRY_TYPE_PAGE: &str = "page";
/// Entry type for media items
pub const ENTRY_TYPE_MEDIA: &str = "media";
/// Entry type for logged events
pub const ENTRY_TYPE_LOG: &str = "log";
/// Entry type for search records
pub const ENTRY_TYPE_SEARCH: &str = "search";
/// Unknown entry type group
pub const ENTRY_TYPE_UNKNOWN: &str = "unknown";

pub const ENTRY_ID_NO_PARENT: i32 = 0;
