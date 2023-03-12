mod app;
mod allocator;
mod consts;
mod date;
mod db;
mod error;
mod post;
mod user;

use crate::db::{parse_csv, write_to_csv};
use crate::post::{EntryType, OX_Post};

fn main() {
    let mut app = app::App::init("Rusty CMS App".to_string());

    // Start editing new post - imitate a new post
    let mut post = OX_Post::draft(EntryType::Post);
    // We add a title + permalink
    post.add_title("This is a new post".to_string(), true);
    dbg!(&post);

    // Store the post
    let _store = db::store(&post);
}
