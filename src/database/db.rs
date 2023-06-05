use crate::database::consts::{
    DB_TABLE_ACCOUNTS, DB_TABLE_ACCOUNT_META, DB_TABLE_POSTS, DB_TABLE_POST_META,
};
use std::fmt::{Display, Formatter};

#[allow(non_camel_case_types)]
pub enum DB_Table {
    Posts,
    #[allow(dead_code)]
    PostMeta,
    Accounts,
    #[allow(dead_code)]
    AccountMeta,
}
impl Into<String> for DB_Table {
    fn into(self) -> String {
        match self {
            DB_Table::Posts => DB_TABLE_POSTS.to_string(),
            DB_Table::PostMeta => DB_TABLE_POST_META.to_string(),
            DB_Table::Accounts => DB_TABLE_ACCOUNTS.to_string(),
            DB_Table::AccountMeta => DB_TABLE_ACCOUNT_META.to_string(),
        }
    }
}
impl Display for DB_Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DB_Table::Posts => write!(f, "{}", DB_TABLE_POSTS),
            DB_Table::PostMeta => write!(f, "{}", DB_TABLE_POST_META),
            DB_Table::Accounts => write!(f, "{}", DB_TABLE_ACCOUNTS),
            DB_Table::AccountMeta => write!(f, "{}", DB_TABLE_ACCOUNT_META),
        }
    }
}

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
