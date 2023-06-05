use crate::consts;
use crate::database::columns::{
    COL_INDEX_POST_CONTENT, COL_INDEX_POST_DATE_MODIFIED, COL_INDEX_POST_DATE_PUBLISH,
    COL_INDEX_POST_EXCERPT, COL_INDEX_POST_ID, COL_INDEX_POST_ID_AUTHOR, COL_INDEX_POST_PARENT,
    COL_INDEX_POST_SLUG, COL_INDEX_POST_STATUS, COL_INDEX_POST_TITLE, COL_INDEX_POST_TYPE,
};
use crate::database::db::DB_Table;
use crate::entry::entry_type::{get_entry_type_variant, EntryType};
use crate::entry::post::{EntryID, GB_Post};
use crate::entry::status::{get_entry_status_variant, EntryStatus};
use crate::errors::error_handler::SqlxError;
use crate::users::user::UserID;
use chrono::NaiveDateTime;
use sqlx::postgres::PgRow;
use sqlx::{Error, PgPool, Row};

#[derive(Debug)]
pub enum GB_QueryArg {
    AuthorID(Vec<UserID>),
    EntryID(Vec<u32>),
    Title(String),
}

#[derive(Debug)]
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
                let the_post: GB_Post = row.into();
                self.results.push(the_post);
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

// Turn PgRow into GB_Post
impl Into<GB_Post> for PgRow {
    fn into(self) -> GB_Post {
        // Underscores' meaning here:
        // we don't need to specify a default/fallback value because the cell will never be empty

        // IDs
        let post_id = self.get::<i32, _>(COL_INDEX_POST_ID) as u32;
        let author_id = self.get::<i32, _>(COL_INDEX_POST_ID_AUTHOR) as u32;
        let parent_id = self
            .try_get(COL_INDEX_POST_PARENT)
            .ok()
            .unwrap_or(consts::ENTRY_ID_NO_PARENT) as u32;

        // Entry type
        let entry_type_as_str: &str = self.get(COL_INDEX_POST_TYPE);
        let the_type: EntryType = get_entry_type_variant(entry_type_as_str);

        // Publish date
        let date_publish: NaiveDateTime = self.get::<NaiveDateTime, _>(COL_INDEX_POST_DATE_PUBLISH);
        let date_publish = date_publish.to_string();

        // Modified date
        let date_modified: NaiveDateTime =
            self.get::<NaiveDateTime, _>(COL_INDEX_POST_DATE_MODIFIED);
        let date_modified = date_modified.to_string();

        // Entry status
        let entry_status_as_str: &str = self.get(COL_INDEX_POST_STATUS);
        let status: EntryStatus = get_entry_status_variant(entry_status_as_str, &the_type);

        GB_Post {
            id: EntryID(post_id),
            id_author: UserID(author_id),
            id_parent: Some(EntryID(parent_id)),
            date_publish,
            date_modified,
            slug: self.get(COL_INDEX_POST_SLUG),
            the_type,
            status,
            title: self.get(COL_INDEX_POST_TITLE),
            excerpt: self.get(COL_INDEX_POST_EXCERPT),
            content: self.get(COL_INDEX_POST_CONTENT),
            password: None,
        }
    }
}
