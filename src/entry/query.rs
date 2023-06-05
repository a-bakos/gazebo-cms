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
pub struct GB_Query(Vec<GB_QueryArg>, Vec<GB_Post>);

// todo rename GB_Post to GB_Entry

impl GB_Query {
    pub fn new(args: Vec<GB_QueryArg>) -> GB_Query {
        GB_Query(args, Vec::new())
    }

    pub fn run(&self) -> Result<Option<Vec<GB_Post>>, Error> {
        // todo
        println!("Coming from GB Query run()");
        // vec of params = self.0

        Ok(Some(Vec::new()))
    }
}

fn query_experiment() {
    let args = vec![GB_QueryArg::EntryID(vec![1, 2, 3])];
    let query = GB_Query::new(args);
    query.run();
}
