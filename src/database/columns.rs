// Posts table

pub const COL_INDEX_POST_ID: &str = "id";
pub const COL_INDEX_POST_ID_AUTHOR: &str = "author_id";
pub const COL_INDEX_POST_PARENT: &str = "parent_id";
pub const COL_INDEX_POST_DATE_PUBLISH: &str = "date_publish";
pub const COL_INDEX_POST_DATE_MODIFIED: &str = "date_modified";
pub const COL_INDEX_POST_SLUG: &str = "slug";
pub const COL_INDEX_POST_TYPE: &str = "post_type";
pub const COL_INDEX_POST_TITLE: &str = "title";
pub const COL_INDEX_POST_EXCERPT: &str = "excerpt";
pub const COL_INDEX_POST_CONTENT: &str = "content";
pub const COL_INDEX_POST_PASSWORD: &str = "password";
pub const COL_INDEX_POST_STATUS: &str = "status";

#[allow(dead_code)]
pub const COLUMNS_POSTS: [(&str, &str); 12] = [
    ("ID", COL_INDEX_POST_ID),
    ("ID_AUTHOR", COL_INDEX_POST_ID_AUTHOR),
    ("PARENT", COL_INDEX_POST_PARENT),
    ("DATE_PUBLISH", COL_INDEX_POST_DATE_PUBLISH),
    ("DATE_MODIFIED", COL_INDEX_POST_DATE_MODIFIED),
    ("SLUG", COL_INDEX_POST_SLUG),
    ("POST_TYPE", COL_INDEX_POST_TYPE),
    ("TITLE", COL_INDEX_POST_TITLE),
    ("EXCERPT", COL_INDEX_POST_EXCERPT),
    ("CONTENT", COL_INDEX_POST_CONTENT),
    ("PASSWORD", COL_INDEX_POST_PASSWORD),
    ("STATUS", COL_INDEX_POST_STATUS),
];

// Accounts / Users table

pub const COL_INDEX_ACCOUNT_ID: &str = "id";
pub const COL_INDEX_ACCOUNT_EMAIL: &str = "email";
pub const COL_INDEX_ACCOUNT_PASSWORD: &str = "password";
pub const COL_INDEX_ACCOUNT_ROLE: &str = "role";
pub const COL_INDEX_ACCOUNT_LOGIN: &str = "login";
pub const COL_INDEX_ACCOUNT_REGISTERED: &str = "registered";
pub const COL_INDEX_ACCOUNT_LAST_LOGIN: &str = "last_login";
pub const COL_INDEX_ACCOUNT_UUID: &str = "uuid";

// Accounts meta

pub const COL_INDEX_ACCOUNT_META_ID: &str = "account_meta_id";
pub const COL_INDEX_ACCOUNT_META_USER_ID: &str = "user_id";
pub const COL_INDEX_ACCOUNT_META_KEY: &str = "meta_key";
pub const COL_INDEX_ACCOUNT_META_VALUE: &str = "meta_value";

#[allow(dead_code)]
pub const COLUMNS_USERS: [(&str, &str); 8] = [
    ("ID", COL_INDEX_ACCOUNT_ID),
    ("LOGIN_NAME", COL_INDEX_ACCOUNT_LOGIN),
    ("EMAIL", COL_INDEX_ACCOUNT_EMAIL),
    ("ROLE", COL_INDEX_ACCOUNT_ROLE),
    ("PASSWORD", COL_INDEX_ACCOUNT_PASSWORD),
    ("REGISTERED", COL_INDEX_ACCOUNT_REGISTERED),
    ("LAST_LOGIN", COL_INDEX_ACCOUNT_LAST_LOGIN),
    ("UUID", COL_INDEX_ACCOUNT_UUID),
];

#[allow(dead_code)]
pub const COLUMNS_USER_META: [(&str, &str); 4] = [
    ("META_ID", COL_INDEX_ACCOUNT_META_ID),
    ("USER_ID", COL_INDEX_ACCOUNT_META_USER_ID),
    ("META_KEY", COL_INDEX_ACCOUNT_META_KEY),
    ("META_VALUE", COL_INDEX_ACCOUNT_META_VALUE),
];

// Media table
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_ID: &str = "id";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_UPLOADER_ID: &str = "uploader";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_ATTACHED_TO: &str = "attached_to";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_DATE_PUBLISH: &str = "date_publish";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_DATE_MODIFIED: &str = "date_modified";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_SLUG: &str = "slug";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_TITLE: &str = "title";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_DESCRIPTION: &str = "description";
#[allow(dead_code)]
pub const COL_INDEX_MEDIA_ALT_TEXT: &str = "alt_text";

#[allow(dead_code)]
pub const COLUMNS_MEDIA: [(&str, &str); 9] = [
    ("ID", COL_INDEX_MEDIA_ID),
    ("UPLOADER_ID", COL_INDEX_MEDIA_UPLOADER_ID),
    ("ATTACHED_TO", COL_INDEX_MEDIA_ATTACHED_TO),
    ("DATE_PUBLISH", COL_INDEX_MEDIA_DATE_PUBLISH),
    ("DATE_MODIFIED", COL_INDEX_MEDIA_DATE_MODIFIED),
    ("SLUG", COL_INDEX_MEDIA_SLUG),
    ("TITLE", COL_INDEX_MEDIA_TITLE),
    ("DESCRIPTION", COL_INDEX_MEDIA_DESCRIPTION),
    ("ALT_TEXT", COL_INDEX_MEDIA_ALT_TEXT),
];

// Log table
pub const COL_INDEX_LOG_ID: &str = "id";
pub const COL_INDEX_LOG_ACCOUNT_ID: &str = "account_id";
pub const COL_INDEX_LOG_ACCOUNT_EMAIL: &str = "account_email";
pub const COL_INDEX_LOG_EVENT_CODE: &str = "event_code";
pub const COL_INDEX_LOG_EVENT_DATE: &str = "event_date";
pub const COL_INDEX_LOG_SUBJECT_ID: &str = "subject_id";
pub const COL_INDEX_LOG_SUBJECT_URL: &str = "subject_url";
pub const COL_INDEX_LOG_SUBJECT_TITLE: &str = "subject_title";
pub const COL_INDEX_LOG_SUBJECT_DESCRIPTION: &str = "subject_description";
pub const COL_INDEX_LOG_SUBJECT_FROM: &str = "subject_from";
pub const COL_INDEX_LOG_SUBJECT_TO: &str = "subject_to";

#[allow(dead_code)]
pub const COLUMNS_LOG: [(&str, &str); 11] = [
    (COL_INDEX_LOG_ID, "id"),
    (COL_INDEX_LOG_ACCOUNT_ID, "account_id"),
    (COL_INDEX_LOG_ACCOUNT_EMAIL, "account_email"),
    (COL_INDEX_LOG_EVENT_CODE, "event_code"),
    (COL_INDEX_LOG_EVENT_DATE, "event_date"),
    (COL_INDEX_LOG_SUBJECT_ID, "subject_id"),
    (COL_INDEX_LOG_SUBJECT_URL, "subject_url"),
    (COL_INDEX_LOG_SUBJECT_TITLE, "subject_title"),
    (COL_INDEX_LOG_SUBJECT_DESCRIPTION, "subject_description"),
    (COL_INDEX_LOG_SUBJECT_FROM, "subject_from"),
    (COL_INDEX_LOG_SUBJECT_TO, "subject_to"),
];

//
// #[allow(dead_code)]
// pub fn get_columns(table: DB_Table) -> HashMap<String, usize> {
//     match table {
//         DB_Table::Posts => {
//             let mut columns: HashMap<String, usize> = HashMap::new();
//             for (col_name, col_index) in COLUMNS_POSTS.iter() {
//                 columns.insert(col_name.to_string(), *col_index);
//             }
//             columns
//         }
//         DB_Table::Accounts => {
//             let mut columns: HashMap<String, String> = HashMap::new();
//             for (col_name, col_index) in COLUMNS_USERS.iter() {
//                 columns.insert(col_name.to_string(), col_index.to_string());
//             }
//             columns
//         }
//     }
// }
