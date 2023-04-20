// This file contains functions that mimic certain sequences / processes in the application.
// Eg. Registering a user, logging in as a user, creating a post, etc.
// This way we can keep the main file clean.

use crate::app::App;
use crate::database::db;
use crate::{
    posts::{entry_type::EntryType, functions as post_functions, post::OX_Post},
    users,
    users::{user::User, user_manager},
};

pub(crate) struct Imitate {}

impl Imitate {
    pub fn update_app_defaults(app: &mut App) {
        app.change_admin_email("admin@example.com");
        app.change_app_name("THE RUST CMS");
    }

    pub fn register_user() {
        // Let's create a new user and insert it
        let test_user = User::new(
            "First".to_string(),
            "Last".to_string(),
            "testuser".to_string(),
            "test@test.com".to_string(),
            users::roles::UserRole::Admin,
            "A345678B".to_string(),
        );
        let is_user_inserted: bool = User::insert(test_user, true);

        dbg!(is_user_inserted);
    }

    // Mimic a user login request
    pub fn user_login(app: &mut App) {
        User::login(app, "test@test.com", "password");
        dbg!(&app.users);
    }

    pub fn get_user_by_email() {
        let getuser = user_manager::get_user_by_email("test@test.com");
        dbg!(getuser.unwrap());
    }

    pub fn add_posts(app: &mut App) {
        // Imitate editing a new post - Eg. User clicks on a "add/create new post" button
        let mut post = OX_Post::draft(app, EntryType::Post);

        // User adds a title to the post with permalink
        // (The title contains special characters which will be treated when the permalink is generated)
        post.add_title(
            "This is the ultimate post of an item @ new #posts & something of value".to_string(),
            true,
        );

        // Imitate editing a second new post creation
        let mut post_2 = OX_Post::draft(app, EntryType::Post);
        // User adds a title to the posts (permalink auto-generated)
        post_2.add_title("This is a second post".to_string(), true);

        // The storage methods will be part of the OX_Post routine
        // ie. store post + update post
        let to_store: Vec<&OX_Post> = vec![&post, &post_2];

        #[allow(clippy::let_unit_value)]
        let _store = db::store_post(to_store);
    }

    pub fn get_post_by_id() {
        #[allow(clippy::let_unit_value)]
        let get_post: Option<OX_Post> = post_functions::get_post_by_id(1).unwrap();
        dbg!(&get_post);
    }

    pub fn get_all_posts() {
        #[allow(clippy::let_unit_value)]
        let all_posts = post_functions::get_all_posts();
        dbg!(all_posts.unwrap());
    }
}
