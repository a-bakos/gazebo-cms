mod allocator;
mod consts;
mod date;
mod db;
mod post;
mod user;

use crate::post::{EntryType, OX_Post};

fn main() {
    let mut post = OX_Post::draft(EntryType::Post);
    dbg!(&post);

    let _store = db::store();
}
