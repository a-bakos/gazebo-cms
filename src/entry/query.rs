use crate::entry::post::{EntryID, GB_Post};
use crate::users::user::UserID;
use sqlx::Error;

pub enum GB_QueryArg {
    AuthorID(Vec<UserID>),
    EntryID(Vec<EntryID>),
}

pub struct GB_Query(Vec<GB_QueryArg>);

// todo rename GB_Post to GB_Entry

impl GB_Query {
    pub fn run(&self) -> Result<Vec<GB_Post>, Error> {
        // todo
        Ok(vec![GB_Post::new()])
    }
}
