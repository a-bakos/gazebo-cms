use crate::{
    consts,
    database::{
        columns::{
            COL_INDEX_POST_CONTENT, COL_INDEX_POST_DATE_MODIFIED, COL_INDEX_POST_DATE_PUBLISH,
            COL_INDEX_POST_EXCERPT, COL_INDEX_POST_ID, COL_INDEX_POST_ID_AUTHOR,
            COL_INDEX_POST_PARENT, COL_INDEX_POST_SLUG, COL_INDEX_POST_STATUS,
            COL_INDEX_POST_TITLE, COL_INDEX_POST_TYPE,
        },
        db::DB_Table,
    },
    entry::{
        entry_id::EntryID,
        entry_type::{get_entry_type_variant, EntryType},
        gb_post::GB_Post,
        status::{get_entry_status_variant, EntryStatus},
    },
    errors::error_handler::SqlxError,
    traits::RowTransformer,
    users::user::UserID,
};

use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, PgPool, Row};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GB_QueryArg {
    #[allow(dead_code)]
    AuthorID(Vec<UserID>),
    EntryID(Vec<u32>),
    #[allow(dead_code)]
    Title(String),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct GB_Query {
    args: Vec<GB_QueryArg>,
    results: Vec<GB_Post>,
    pool: PgPool, // Will be a pool clone
}

impl GB_Query {
    pub fn new(args: Vec<GB_QueryArg>, pool: PgPool) -> Self {
        Self {
            args,
            results: Vec::new(),
            pool,
        }
    }

    // Collect results into self.results
    pub async fn run(&mut self) -> Result<bool, SqlxError> {
        // query will be highly varied based on self.args!
        println!("{:?}", self.args);

        let query = format!("SELECT * FROM {} WHERE post_type = $1", DB_Table::Posts);
        match sqlx::query(&query)
            .bind("post")
            .map(|row: PgRow| {
                let gb_post = GB_Post::transform(&row);
                self.results.push(gb_post);
            })
            .fetch_all(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(SqlxError(e)),
        }
    }

    pub fn get_results(&self) -> &Vec<GB_Post> {
        &self.results
    }
}
