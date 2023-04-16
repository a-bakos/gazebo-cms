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

mod run_sequence;

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

    run_sequence::Process::register_user(&mut app);

    println!("Getting user table row");
    db::Database::get_row(DB_Table::Users, 0);

    // Mimic a user login request
    users::user::User::login(&mut app, "test@test.com");
    dbg!(&app.users);

    let getuser = users::user_manager::get_user_by_email("test@test.com");
    dbg!(getuser);

    run_sequence::Process::add_posts(&mut app);

    #[allow(clippy::let_unit_value)]
    //let _get_post: Option<OX_Post> = post_functions::get_post_by_id(1).unwrap();
    // dbg!(&_get_post);

    // dbg!(crate::database::columns::get_columns());

    // Check the App state
    //dbg!(&app.resources);

    dbg!(&app.start);

    #[allow(clippy::let_unit_value)]
    let _all_posts = post_functions::get_all_posts();
    // dbg!(_all_posts);
}
