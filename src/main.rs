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
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::POST, Method::GET]);

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

    let get_users_html = warp::get()
        .and(warp::path("usershtml"))
        .and(warp::path::end()) // ::end() closes the URI path
        .and(warp::query())
        .and(pool_filter.clone())
        .and_then(get_users_html);

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

    let index = warp::path::end() // Match the root path ("/")
        .and(warp::get()) // Handle GET requests
        .and(pool_filter.clone())
        .and_then(get_index_html);

    let routes = get_user
        .or(index)
        .or(get_users_html)
        .or(registration)
        .or(login)
        .or(delete_user)
        .or(get_post)
        .or(insert_post)
        .with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;

    Ok(())
}

async fn get_index_html(_pool: PgPool) -> Result<impl Reply, Infallible> {
    let html = format!(
        r#"<html><head><title>Gazebo CMS index page</title></head>
            <body>
            <h1>Login</h1>
            <form method="post" action="/login">
                <label for="username">Username:</label>
                <input type="text" id="username" name="username" required><br>
                <label for="password">Password:</label>
                <input type="password" id="password" name="password" required><br>
                <input type="submit" value="Login">
            </form>
        </body></html>"#
    );

    let response = Response::builder()
        .header("content-type", "text/html")
        .body(html)
        .unwrap();
    Ok(response.into_response())
}

// This is just an experimental feature
// http://localhost:1337/users?name=what&age=whatwhat
async fn get_users_html(
    params: HashMap<String, String>,
    _pool: PgPool,
) -> Result<impl Reply, Infallible> {
    let name = params
        .get("name")
        .unwrap_or(&"GAZEBO".to_owned())
        .to_owned();
    let html = format!(
        r#"<html><head><title>Users Page</title></head><body><h1>Hello, {}!</h1></body></html>"#,
        name
    );

    let response = Response::builder()
        .header("content-type", "text/html")
        .body(html)
        .unwrap();
    Ok(response.into_response())
}
