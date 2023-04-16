// This file contains functions that mimic certain sequences / processes in the application.
// Eg. Registering a user, logging in as a user, creating a post, etc.
// This way we can keep the main file clean.

use crate::app::App;
use crate::database::db;
use crate::posts::post::OX_Post;
use crate::{posts, users};

pub(crate) struct Imitate {}

impl Imitate {
    pub fn register_user(app: &mut App) -> bool {
        // Let's create a new user and insert it
        let test_user = crate::users::user::User::new(
            "First".to_string(),
            "Last".to_string(),
            "testuser".to_string(),
            "test@test.com".to_string(),
            users::roles::UserRole::Admin,
            "12345678".to_string(),
        );
        let is_user_inserted: bool = users::user::User::insert(app, test_user);

        is_user_inserted
    }

    pub fn add_posts(app: &mut App) {
        // Imitate editing a new post - Eg. User clicks on a "add/create new post" button
        let mut post = OX_Post::draft(app, posts::entry_type::EntryType::Post);

        // User adds a title to the post with permalink
        // (The title contains special characters which will be treated when the permalink is generated)
        post.add_title(
            "This is the ultimate post of an item @ new #posts & something of value".to_string(),
            true,
        );

        // Imitate editing a second new post creation
        let mut post_2 = OX_Post::draft(app, posts::entry_type::EntryType::Post);
        // User adds a title to the posts (permalink auto-generated)
        post_2.add_title("This is a second post".to_string(), true);

        // The storage methods will be part of the OX_Post routine
        // ie. store post + update post
        let to_store: Vec<&OX_Post> = vec![&post, &post_2];

        #[allow(clippy::let_unit_value)]
        let _store = db::store_post(to_store);
    }
}
