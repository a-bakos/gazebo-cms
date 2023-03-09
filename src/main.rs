mod allocator;
mod consts;
mod date;
mod db;
mod error;
mod post;
mod user;

use crate::post::{EntryType, OX_Post};

fn main() {
    // Start editing new post
    let mut post = OX_Post::draft(EntryType::Post);

    // Update post with post data
    // post = post.update();

    dbg!(&post);

    let _store = db::store(&post);
}
