use gazebo_core_common::account::gb_account::AccountID;

/// App default
pub const DEFAULT_APP_NAME: &str = "Gazebo CMS";
pub const DEFAULT_APP_ADMIN_EMAIL: &str = "change_this@gazebocms.email";
pub const VERSION: &str = "0.0.402";

#[allow(dead_code)]
pub const SYSTEM_USER_ID: AccountID = AccountID(0);

/// Permalink defaults
pub const PERMALINK_MAX_ALLOWED_LENGTH: usize = 1024;
pub const DEFAULT_PERMALINK_SEPARATOR: &str = "-";
pub const DEFAULT_PERMALINK_LIMIT: usize = 60;
pub const DEFAULT_PERMALINK_STOP_WORDS: [&str; 4] = ["and", "the", "of", "a"];
pub const DEFAULT_PERMALINK_NOT_ALLOWED_CHARS: [&str; 9] =
    ["&", "#", "?", "%", "<", ">", "\"", "'", "/"];

pub const ENTRY_ID_NO_PARENT: i32 = 0;
