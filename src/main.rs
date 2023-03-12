mod app;
mod allocator;
mod consts;
mod date;
mod db;
mod error;
mod post;
mod user;

use crate::post::{EntryType, OX_Post};

fn main() {
    let mut app = app::App::init("Rusty CMS Experiment App".to_string());

    // Start editing new post - imitate a new post
    let mut post = OX_Post::draft(&mut app, EntryType::Post);
    // We add a title + permalink
    post.add_title("This is a new post".to_string(), true);
    dbg!(&post);

    let mut post_2 = OX_Post::draft(&mut app, EntryType::Post);
    post_2.add_title("This is a second post".to_string(), true);

    dbg!(&app.resources);

    // Store the post
    let _store = db::store(&post);
}
