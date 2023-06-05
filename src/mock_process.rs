// This file contains functions that mimic certain sequences / processes in the application.
// Eg. Registering a user, logging in as a user, creating a post, etc.
// This way we can keep the main file clean.

use crate::app::App;
use crate::entry::query::{GB_Query, GB_QueryArg};
use crate::entry::{entry_type::EntryType, post::GB_Post};
use sqlx::PgPool;

pub(crate) struct Imitate {}

impl Imitate {
    #[allow(dead_code)]
    pub fn update_app_defaults(app: &mut App) {
        app.change_admin_email("admin@example.com");
        app.change_app_name("THE RUST CMS");
    }

    #[allow(dead_code)]
    pub fn register_user() {
        todo!()
        // Let's create a new user and insert it
        // let test_user = User::new(
        //     "testuser".to_string(),
        //     "test@test.com".to_string(),
        //     users::roles::UserRole::Admin,
        //     "A345678B".to_string(),
        // );
        //let is_user_inserted: bool = User::insert(test_user, true);
        //dbg!(is_user_inserted);
    }

    // Mimic a user login request
    #[allow(dead_code)]
    pub fn user_login(_app: &mut App) {
        todo!()
        // User::login(app, "test@test.com", "password");
        // dbg!(&app.users);
    }

    #[allow(dead_code)]
    pub fn get_user_by_email() {
        todo!()
        // let user_email = "test@test.com";
        // if get_user_by_email(user_email).is_ok() {
        //     let getuser = get_user_by_email(user_email);
        //     dbg!(getuser.unwrap());
        // } else {
        //     println!("Cannot get user / can't find file");
        // }
    }

    #[allow(dead_code)]
    pub fn add_posts(app: &mut App) {
        // Imitate editing a new post - Eg. User clicks on a "add/create new post" button
        let mut post = GB_Post::draft(app, EntryType::Post);

        // User adds a title to the post with permalink
        // (The title contains special characters which will be treated when the permalink is generated)
        post.add_title(
            "This is the ultimate post of an item @ new #entry & something of value".to_string(),
            true,
        );

        // Imitate editing a second new post creation
        //let mut post_2 = OX_Post::draft(app, EntryType::Post);
        // User adds a title to the entry (permalink auto-generated)
        //post_2.add_title("This is a second post".to_string(), true);

        //let mut post_3 = OX_Post::draft(app, EntryType::Post);
        //post_3.add_title("This is a second post".to_string(), true);

        //let mut post_4 = OX_Post::draft(app, EntryType::Post);
        //post_4.add_title("This is a second post".to_string(), true);
    }

    #[allow(dead_code)]
    pub fn get_post_by_id() {
        todo!()
        // #[allow(clippy::let_unit_value)]
        // let get_post: Option<GB_Post> = post_functions::get_post_by_id(1).unwrap();
        // dbg!(&get_post);
    }

    #[allow(dead_code)]
    pub fn get_all_posts() {
        todo!()
        //#[allow(clippy::let_unit_value)]
        // let all_posts = post_functions::get_all_posts();
        // dbg!(all_posts.unwrap());
    }

    pub async fn gb_query_get_posts_by_id(pool: PgPool) {
        let args = vec![GB_QueryArg::EntryID(vec![1, 2, 3])];
        let mut query = GB_Query::new(args, pool);
        query.run().await.expect("failed to execute query");
        let query_results = query.get_results();

        println!("{:#?}", query_results);
    }
}
