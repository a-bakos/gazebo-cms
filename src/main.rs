extern crate core;

mod allocator;
mod app;
mod consts;
mod database;
mod datetime;
mod entry;
mod errors;
mod helpers;
mod http;
mod private;
mod routes;
mod url;
mod users;

mod mock_process;

use std::collections::HashMap;
use std::convert::Infallible;

use sqlx::postgres::{PgPool, PgPoolOptions};

use warp::http::Response;
use warp::Reply;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Start the App
    let _app = app::App::init();
    // App started timestamp:
    // dbg!(&app.start);

    // mock_process::Imitate::update_app_defaults(&mut app);

    // DB Setup
    // let pool = db_setup(...).await;
    let db_pass = crate::private::DB_PASS;
    let db_url = &("postgres://postgres:".to_owned() + db_pass + "@localhost/gazebocms");
    println!("{}", db_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    //mock_process::Imitate::gb_query_get_posts_by_id(pool.clone()).await;

    // execute migrations before anything
    sqlx::migrate!().run(&pool.clone()).await?;

    let pool_filter = warp::any().map(move || pool.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(&[Method::PUT, Method::DELETE, Method::POST, Method::GET])
        .build();

    let get_user = warp::get()
        .and(warp::path(url::path::PATH_USER))
        .and(warp::path::param::<i32>())
        .and(warp::path::end()) // ::end() closes the URI path
        // .and(warp::query()) // => this is the params: HashMap<String, String> parameter in the callback
        .and(pool_filter.clone())
        .and_then(routes::user::crud::get_user_by_id);

    let delete_user = warp::delete()
        .and(warp::path(url::path::PATH_USER))
        .and(warp::path::param::<i32>())
        .and(warp::path::end()) // ::end() closes the URI path
        // .and(warp::query()) // => this is the params: HashMap<String, String> parameter in the callback
        .and(pool_filter.clone())
        .and_then(routes::user::crud::delete_user_by_id);

    let registration = warp::post()
        .and(warp::path(url::path::PATH_USER_REGISTRATION))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::user::registration::registration);

    let login = warp::post()
        .and(warp::path(url::path::PATH_USER_LOGIN))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::user::login::login);

    let get_post = warp::get()
        .and(warp::path(url::path::PATH_POST))
        .and(warp::path::param::<i32>())
        .and(warp::path::end()) // ::end() closes the URI path
        // .and(warp::query()) // => this is the params: HashMap<String, String> parameter in the callback
        .and(pool_filter.clone())
        .and_then(routes::post::crud::get_post_by_id);

    let insert_post = warp::post()
        .and(warp::path("post"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::post::crud::insert_post);

    // todo
    //let create_post = warp::post()
    //    .and(warp::path(url::path::PATH_POST_ADD))
    //    .and(warp::path::end())
    //    .and(pool_filter.clone())
    //    .and(warp::body::json())
    //    .and_then(routes::post::crud::insert_post);

    let get_post_title = warp::get()
        .and(warp::path("title"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(routes::post::crud::get_the_title);

    let routes = get_user
        // .or(index)
        .or(registration)
        .or(login)
        .or(delete_user)
        .or(get_post)
        .or(insert_post)
        .or(get_post_title)
        .with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;

    Ok(())
}
