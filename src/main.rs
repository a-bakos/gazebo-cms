extern crate core;

mod allocator;
mod app;
mod consts;
mod database;
mod dates;
mod error;
mod helpers;
mod http;
mod posts;
mod private;
mod url;
mod users;

mod mock_process;

use std::collections::HashMap;
use std::convert::Infallible;
use warp::{http::Method, Filter};

use sqlx::postgres::{PgPool, PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Start the App
    let mut app = app::App::init();
    // App started timestamp:
    // dbg!(&app.start);

    //mock_process::Imitate::update_app_defaults(&mut app);

    // User-related processes
    //mock_process::Imitate::register_user();
    //mock_process::Imitate::user_login(&mut app);
    //mock_process::Imitate::get_user_by_email();

    // Post-related processes
    //mock_process::Imitate::add_posts(&mut app);
    //mock_process::Imitate::get_post_by_id();
    //mock_process::Imitate::get_all_posts();

    let db_pass = crate::private::DB_PASS;
    let db_url = &("postgres://postgres:".to_owned() + db_pass + "@localhost/gazebocms");
    println!("{}", db_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // execute migrations before anything
    sqlx::migrate!().run(&pool.clone()).await?;

    let pool_filter = warp::any().map(move || pool.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::POST, Method::GET]);

    let get_users = warp::get()
        .and(warp::path("users"))
        .and(warp::path::end()) // ::end() closes the URI path
        .and(warp::query())
        .and(pool_filter.clone())
        .and_then(get_users);

    // let hello = warp::path("hello").map(|| format!("Hello, Gazebo CMS!"));
    // warp::serve(hello).run(([127, 0, 0, 1], 1337)).await;

    let routes = get_users.with(cors);
    // .recover(return_error)
    // .with(warp::trace::request());
    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;

    Ok(())
}
// http://localhost:1337/users?name=what&age=whatwhat
async fn get_users(
    params: HashMap<String, String>,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    // dummy implementation
    let message = format!("Get all users with query params: {:?}", params);
    Ok(warp::reply::with_status(
        message,
        warp::http::StatusCode::OK,
    ))
}
