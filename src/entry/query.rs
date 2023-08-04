use crate::{
    database::db::DB_Table, entry::gb_post::GB_Post, errors::error_handler::SqlxError,
    traits::RowTransformer,
};
use gazebo_core_common::account::gb_account::AccountID;
use sqlx::{postgres::PgRow, PgPool};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GB_QueryArg {
    #[allow(dead_code)]
    AuthorID(Vec<AccountID>),
    #[allow(dead_code)]
    EntryID(Vec<u32>),
    #[allow(dead_code)]
    Title(String),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct GB_Query {
    #[allow(dead_code)]
    args: Vec<GB_QueryArg>,
    #[allow(dead_code)]
    results: Vec<GB_Post>,
    #[allow(dead_code)]
    pool: PgPool, // Will be a pool clone
}

impl GB_Query {
    #[allow(dead_code)]
    pub fn new(args: Vec<GB_QueryArg>, pool: PgPool) -> Self {
        Self {
            args,
            results: Vec::new(),
            pool,
        }
    }

    // Collect results into self.results
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn get_results(&self) -> &Vec<GB_Post> {
        &self.results
    }
}
