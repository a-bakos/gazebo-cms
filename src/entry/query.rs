use crate::entry::post::{EntryID, GB_Post};
use crate::users::user::UserID;
use sqlx::{Error, PgPool};

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
    pool: PgPool, // pool clone
}

// todo rename GB_Post to GB_Entry

impl GB_Query {
    pub fn new(args: Vec<GB_QueryArg>, pool: PgPool) -> Self {
        Self {
            args,
            results: Vec::new(),
            pool,
        }
    }

    pub fn run(&self) -> Result<bool, Error> {
        // todo
        println!("Coming from GB Query run()");
        // collect results into self.results

        // SQLX query here

        Ok(true)
    }

    pub fn get_results(&self) -> &Vec<GB_Post> {
        &self.results
    }
}
