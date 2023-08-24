extern crate core;

mod app;
mod auth;
mod consts;
mod database;
mod entry;
mod errors;
mod http;
mod private;
mod routes;
mod traits;
mod url;
mod users;

use sqlx::postgres::PgPoolOptions;
use warp::{http::Method, Filter};

use crate::auth::generate_token;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Start the App
    let app = app::App::init();
    // App started timestamp:
    dbg!(&app.start);
    //app.change_admin_email("admin@example.com");
    //app.change_app_name("Gazebo CMS");
    dbg!(&app);

    // DB Setup
    // let pool = db_setup(...).await;
    let db_pass = crate::private::DB_PASS;
    let db_url = &("postgres://postgres:".to_owned() + db_pass + "@localhost/gazebocms");
    println!("{}", db_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Execute migrations before anything
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
        .and(pool_filter.clone())
        .and_then(routes::accounts::crud::get_user_by_id);

    let get_users = warp::get()
        .and(warp::path(url::path::PATH_USER))
        .and(warp::path::end()) // ::end() closes the URI path
        .and(pool_filter.clone())
        .and_then(routes::accounts::crud::get_all_accounts);

    let delete_user = warp::delete()
        .and(warp::path(url::path::PATH_USER))
        .and(warp::path::param::<i32>())
        .and(warp::path::end()) // ::end() closes the URI path
        .and(pool_filter.clone())
        .and_then(routes::accounts::crud::delete_user_by_id);

    let registration = warp::post()
        .and(warp::path(url::path::PATH_USER_REGISTRATION))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::accounts::crud::add);

    let login = warp::post()
        .and(warp::path(url::path::PATH_USER_LOGIN))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::accounts::login::login);

    let auth_token = warp::post()
        .and(warp::path(url::path::PATH_USER_AUTH_TOKEN))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::accounts::login::token_auth);

    let get_post = warp::get()
        .and(warp::path(url::path::PATH_POST))
        .and(warp::path::param::<i32>())
        .and(warp::path::end()) // ::end() closes the URI path
        .and(pool_filter.clone())
        .and_then(routes::post::crud::get_post_by_id);

    let get_posts = warp::get()
        .and(warp::path(url::path::PATH_POSTS))
        .and(warp::path::end()) // ::end() closes the URI path
        // .and(warp::query()) // => this is the params: HashMap<String, String> parameter in the callback
        .and(pool_filter.clone())
        .and_then(routes::post::crud::get_posts);

    let insert_post = warp::post()
        .and(warp::path("post"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::post::crud::insert_post);

    let delete_post = warp::delete()
        .and(warp::path(url::path::PATH_POST))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(routes::post::crud::delete_post);

    let update_post = warp::put()
        .and(warp::path(url::path::PATH_POST))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(routes::post::crud::update_entry_single_param);

    // todo
    //let create_post = warp::post()
    //    .and(warp::path(url::path::PATH_POST_ADD))
    //    .and(warp::path::end())
    //    .and(pool_filter.clone())
    //    .and(warp::body::json())
    //    .and_then(routes::post::crud::insert_post);

    let get_post_title = warp::get()
        .and(warp::path(url::path::PATH_POST))
        .and(warp::path(url::path::PATH_TITLE))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(routes::post::crud::get_the_title);

    let routes = get_user
        .or(get_users)
        .or(registration)
        .or(login)
        .or(auth_token)
        .or(delete_user)
        .or(get_post)
        .or(get_posts)
        .or(insert_post)
        .or(delete_post)
        .or(update_post)
        .or(get_post_title)
        .with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;

    Ok(())
}
