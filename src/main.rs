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

use warp::Filter;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Start the App
    let mut app = app::App::init();
    // App started timestamp:
    // dbg!(&app.start);

    mock_process::Imitate::update_app_defaults(&mut app);

    // User-related processes
    mock_process::Imitate::register_user();
    mock_process::Imitate::user_login(&mut app);
    mock_process::Imitate::get_user_by_email();

    // Post-related processes
    //mock_process::Imitate::add_posts(&mut app);
    //mock_process::Imitate::get_post_by_id();
    //mock_process::Imitate::get_all_posts();

    //let hello = warp::path("hello").map(|| format!("Hello, Gazebo CMS!"));
    //warp::serve(hello).run(([127, 0, 0, 1], 1337)).await;

    let db_pass = crate::private::DB_PASS;
    let db_url = &("postgres://postgres:".to_owned() + db_pass + "@localhost/gazebocms");
    println!("{}", db_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // execute migrations before anything
    sqlx::migrate!().run(&pool).await?;

    Ok(())
}
