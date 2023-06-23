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

// Accounts meta

pub const COL_INDEX_ACCOUNT_META_ID: &str = "account_meta_id";
pub const COL_INDEX_ACCOUNT_META_USER_ID: &str = "user_id";
pub const COL_INDEX_ACCOUNT_META_KEY: &str = "meta_key";
pub const COL_INDEX_ACCOUNT_META_VALUE: &str = "meta_value";

#[allow(dead_code)]
pub const COLUMNS_USERS: [(&str, &str); 7] = [
    ("ID", COL_INDEX_ACCOUNT_ID),
    ("LOGIN_NAME", COL_INDEX_ACCOUNT_LOGIN),
    ("EMAIL", COL_INDEX_ACCOUNT_EMAIL),
    ("ROLE", COL_INDEX_ACCOUNT_ROLE),
    ("PASSWORD", COL_INDEX_ACCOUNT_PASSWORD),
    ("REGISTERED", COL_INDEX_ACCOUNT_REGISTERED),
    ("LAST_LOGIN", COL_INDEX_ACCOUNT_LAST_LOGIN),
];

#[allow(dead_code)]
pub const COLUMNS_USER_META: [(&str, &str); 4] = [
    ("META_ID", COL_INDEX_ACCOUNT_META_ID),
    ("USER_ID", COL_INDEX_ACCOUNT_META_USER_ID),
    ("META_KEY", COL_INDEX_ACCOUNT_META_KEY),
    ("META_VALUE", COL_INDEX_ACCOUNT_META_VALUE),
];

// Media table
pub const COL_INDEX_MEDIA_ID: &str = "id";
pub const COL_INDEX_MEDIA_UPLOADER_ID: &str = "uploader";
pub const COL_INDEX_MEDIA_ATTACHED_TO: &str = "attached_to";
pub const COL_INDEX_MEDIA_DATE_PUBLISH: &str = "date_publish";
pub const COL_INDEX_MEDIA_DATE_MODIFIED: &str = "date_modified";
pub const COL_INDEX_MEDIA_SLUG: &str = "slug";
pub const COL_INDEX_MEDIA_TITLE: &str = "title";
pub const COL_INDEX_MEDIA_DESCRIPTION: &str = "description";
pub const COL_INDEX_MEDIA_ALT_TEXT: &str = "alt_text";

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
