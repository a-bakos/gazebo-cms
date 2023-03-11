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
    dbg!(&post);
    // Update post with post data
    //post = post.update();

    post.add_title("This is a new post".to_string(), true);
    dbg!(&post);

    //let _store = db::store(&post);
}
