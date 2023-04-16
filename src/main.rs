extern crate core;

mod allocator;
mod app;
mod consts;
mod database;
mod dates;
mod error;
mod http;
mod posts;
mod url;
mod users;

mod mock_process;

use crate::database::db;
use crate::database::db::DB_Table;
use crate::posts::post::OX_Post;
use posts::functions as post_functions;

fn main() {
    // Start the App
    let mut app = app::App::init(
        "Rusty CMS Experiment App".to_string(),
        consts::VERSION.to_string(),
    );
    // App started timestamp:
    // dbg!(&app.start);

    // User-related processes
    mock_process::Imitate::register_user(&mut app);
    mock_process::Imitate::user_login(&mut app);
    mock_process::Imitate::get_user_by_email();

    // Post-related processes
    mock_process::Imitate::add_posts(&mut app);
    mock_process::Imitate::get_post_by_id();
    mock_process::Imitate::get_all_posts();

    println!("Getting user table row");
    db::Database::get_row(DB_Table::Users, 0);

    // dbg!(crate::database::columns::get_columns());

    // Check the App state
    //dbg!(&app.resources);
}
