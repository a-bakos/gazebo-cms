use crate::entry::post::{EntryID, GB_Post};
use crate::users::user::UserID;
use sqlx::Error;

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
}

// todo rename GB_Post to GB_Entry

impl GB_Query {
    pub fn new(args: Vec<GB_QueryArg>) -> Self {
        Self {
            args,
            results: Vec::new(),
        }
    }

    pub fn run(&self) -> Result<bool, Error> {
        // todo
        println!("Coming from GB Query run()");
        // collect results into self.results

        Ok(true)
    }

    pub fn get_results(&self) -> &Vec<GB_Post> {
        &self.results
    }
}
