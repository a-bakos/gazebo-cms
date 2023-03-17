extern crate core;

mod allocator;
mod app;
mod consts;
mod database;
mod dates;
mod error;
mod posts;
mod users;

use crate::database::db;
use crate::posts::post::OX_Post;

fn main() {
    // Start the App
    let mut app = app::App::init(
        "Rusty CMS Experiment App".to_string(),
        consts::VERSION.to_string(),
    );

    // Imitate editing a new posts - Eg. User clicks a "new posts" button
    let mut post = OX_Post::draft(&mut app, posts::entry_type::EntryType::Post);
    // User adds a title to the posts (permalink auto-generated)
    post.add_title("This is a new posts".to_string(), true);

    // Imitate editing a second new posts - Eg. User clicks a "new posts" button
    let mut post_2 = OX_Post::draft(&mut app, posts::entry_type::EntryType::Post);
    // User adds a title to the posts (permalink auto-generated)
    post_2.add_title("This is a second posts".to_string(), true);

    // Check the App state
    dbg!(&app.resources);

    // The storage methods will be part of the OX_Post routine
    let to_store: Vec<&OX_Post> = vec![&post, &post_2];
    let _store = db::store(to_store);
}
