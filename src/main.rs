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

fn main() {
    // Start the App
    let mut app = app::App::init();
    // App started timestamp:
    // dbg!(&app.start);

    app.change_admin_email("admin@example.com");
    app.change_app_name("THE RUST CMS");

    // User-related processes
    mock_process::Imitate::register_user();
    mock_process::Imitate::user_login(&mut app);
    mock_process::Imitate::get_user_by_email();

    // Post-related processes
    mock_process::Imitate::add_posts(&mut app);
    mock_process::Imitate::get_post_by_id();
    mock_process::Imitate::get_all_posts();
}
