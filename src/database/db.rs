use crate::database::consts::{
    DB_TABLE_ACCOUNTS, DB_TABLE_ACCOUNT_META, DB_TABLE_LOG, DB_TABLE_MEDIA, DB_TABLE_PAGES,
    DB_TABLE_PAGE_META, DB_TABLE_POSTS, DB_TABLE_POST_META, DB_TABLE_SEARCH,
};
use std::fmt::{Display, Formatter};

#[allow(non_camel_case_types)]
pub enum DB_Table {
    Posts,
    #[allow(dead_code)]
    PostMeta,
    #[allow(dead_code)]
    Pages,
    #[allow(dead_code)]
    PageMeta,
    #[allow(dead_code)]
    Accounts,
    #[allow(dead_code)]
    AccountMeta,
    #[allow(dead_code)]
    Media,
    #[allow(dead_code)]
    Log,
    #[allow(dead_code)]
    Search,
}

impl From<DB_Table> for String {
    fn from(val: DB_Table) -> Self {
        match val {
            DB_Table::Posts => DB_TABLE_POSTS.to_string(),
            DB_Table::PostMeta => DB_TABLE_POST_META.to_string(),
            DB_Table::Pages => DB_TABLE_PAGES.to_string(),
            DB_Table::PageMeta => DB_TABLE_PAGE_META.to_string(),
            DB_Table::Accounts => DB_TABLE_ACCOUNTS.to_string(),
            DB_Table::AccountMeta => DB_TABLE_ACCOUNT_META.to_string(),
            DB_Table::Media => DB_TABLE_MEDIA.to_string(),
            DB_Table::Log => DB_TABLE_LOG.to_string(),
            DB_Table::Search => DB_TABLE_SEARCH.to_string(),
        }
    }
}
impl Display for DB_Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DB_Table::Posts => write!(f, "{}", DB_TABLE_POSTS),
            DB_Table::PostMeta => write!(f, "{}", DB_TABLE_POST_META),
            DB_Table::Pages => write!(f, "{}", DB_TABLE_PAGES),
            DB_Table::PageMeta => write!(f, "{}", DB_TABLE_PAGE_META),
            DB_Table::Accounts => write!(f, "{}", DB_TABLE_ACCOUNTS),
            DB_Table::AccountMeta => write!(f, "{}", DB_TABLE_ACCOUNT_META),
            DB_Table::Media => write!(f, "{}", DB_TABLE_MEDIA),
            DB_Table::Log => write!(f, "{}", DB_TABLE_LOG),
            DB_Table::Search => write!(f, "{}", DB_TABLE_SEARCH),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct GBDB {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub charset: String,
    pub table_prefix: String,
}

impl GBDB {
    pub fn new(
        name: String,
        user: String,
        password: String,
        host: String,
        charset: String,
        table_prefix: String,
    ) -> Self {
        Self {
            name,
            user,
            password,
            host,
            charset,
            table_prefix,
        }
    }
}
