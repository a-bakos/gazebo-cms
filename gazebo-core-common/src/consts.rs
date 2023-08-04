pub const USER_ROLE_ADMIN: &str = "administrator";
pub const USER_ROLE_EDITOR: &str = "editor";
pub const USER_ROLE_CONTRIBUTOR: &str = "contributor";
pub const USER_ROLE_NOT_FOUND: &str = "not_found";

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

/// Messages
#[allow(dead_code)]
pub const MSG_LOGIN_SUCCESS: &str = "Login successful";

/// Labels
pub const LABEL_NONE: &str = "none";
#[allow(dead_code)]
pub const LABEL_UNKNOWN: &str = "unknown";
#[allow(dead_code)]
pub const LABEL_YES: &str = "yes";
#[allow(dead_code)]
pub const LABEL_NO: &str = "no";
