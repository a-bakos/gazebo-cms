extern crate core;

mod allocator;
mod app;
mod consts;
mod date;
mod db;
mod error;
mod post;
mod user;

use crate::post::{EntryType, OX_Post};

fn main() {
    // Start the App
    let mut app = app::App::init("Rusty CMS Experiment App".to_string());

    // Imitate editing a new post - Eg. User clicks a "new post" button
    let mut post = OX_Post::draft(&mut app, EntryType::Post);
    // User adds a title to the post (permalink auto-generated)
    post.add_title("This is a new post".to_string(), true);

    // Imitate editing a second new post - Eg. User clicks a "new post" button
    let mut post_2 = OX_Post::draft(&mut app, EntryType::Post);
    // User adds a title to the post (permalink auto-generated)
    post_2.add_title("This is a second post".to_string(), true);

    // Check the App state
    dbg!(&app.resources);

    // Store the posts
    let _store = db::store(&post);
    let _store = db::store(&post_2);
}
