mod allocator;
mod date;
mod post;
mod user;

use crate::post::{Entry, EntryType};

const VERSION: &str = "0.0.1";

fn main() {
    let mut post = Entry::draft(EntryType::Post);
    dbg!(&post);
}
