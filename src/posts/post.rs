/*
WP_POST
    [x] public $ID;
    [x] public $post_author = 0;
    [x] public $post_date = '0000-00-00 00:00:00'; // The posts's local publication time.
    public $post_date_gmt = '0000-00-00 00:00:00'; // The posts's GMT publication time.
    [x] public $post_content = '';
    [x] public $post_title = '';
    [x] public $post_excerpt = '';
    [x] public $post_status = 'publish';
    public $comment_status = 'open';
    public $ping_status = 'open'; // Whether pings are allowed
    [x] public $post_password = ''; // The posts's password in plain text.
    [x] public $post_name = ''; // The posts's slug.
    public $to_ping = ''; // URLs queued to be pinged.
    public $pinged = ''; // URLs that have been pinged.
    [x] public $post_modified = '0000-00-00 00:00:00'; // The posts's local modified time.
    public $post_modified_gmt = '0000-00-00 00:00:00'; // The posts's GMT modified time.
    public $post_content_filtered = ''; // A utility DB field for posts content.
    [x] public $post_parent = 0; // ID of a posts's parent posts.
    public $guid = ''; // The unique identifier for a posts, not necessarily a URL, used as the feed GUID.
    public $menu_order = 0; // A field used for ordering posts.
    [x] public $post_type = 'posts';
    public $post_mime_type = ''; // An attachment's mime type.
    public $comment_count = 0; // Cached comment count. A numeric string, for compatibility reasons.
*/

use crate::allocator::{ID_Allocator, ResourceID, ResourceManager, ResourceType};
use crate::app::App;
use crate::database::db::*;
use crate::dates::functions as date_functions;
use crate::posts::entry_type::EntryType;
use crate::users::user::UserID;
use crate::{consts, url};
use std::fmt::{write, Formatter};

#[derive(Debug)]
pub enum PostSpecific {
    Title,
    Permalink,
    AuthorID,
    ParentID,
    // DatePublished,
    Excerpt,
    Content,
    Password,
}

// WP statuses:
// publish
// future
// draft
// pending
// private
// trash
// auto-draft
// inherit
// request-pending
// request-confirmed
// request-failed
// request-completed

#[derive(Debug, Clone)]
pub enum PostStatus {
    Draft,
    Publish,
    Private,
    Trash,
}

impl std::fmt::Display for PostStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PostStatus::Draft => write!(f, "draft"),
            PostStatus::Publish => write!(f, "publish"),
            PostStatus::Private => write!(f, "private"),
            PostStatus::Trash => write!(f, "trash"),
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct OX_Post {
    pub id: EntryID,
    pub id_author: UserID,
    pub id_parent: Option<EntryID>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: Option<String>,
    pub the_type: EntryType,
    pub status: PostStatus,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub password: Option<String>,
}

// New type patterns for IDs
#[derive(Default, Debug, Copy, Clone)]
pub struct EntryID(pub u32);

impl std::fmt::Display for EntryID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for EntryID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl EntryID {
    // get current ID
    #[allow(dead_code)]
    fn get() -> Self {
        EntryID(200)
    }
}

impl ID_Allocator for EntryID {
    fn allocate(app: &mut App) -> Self {
        let resource_entry_id = ResourceManager::get_next_available_id(app, ResourceType::Entry);
        dbg!(&resource_entry_id);
        let entry_id = match resource_entry_id {
            ResourceID::EntryID(id) => EntryID(id),
            // Todo: users ID
            _ => EntryID(0),
        };
        let _ = &app
            .resources
            .add_to_allocated(ResourceType::Entry, resource_entry_id);
        entry_id
    }
}

#[allow(dead_code)]
fn get_next_available_entry_id(app: &mut App) -> EntryID {
    EntryID::allocate(app)
}

#[allow(dead_code)]
fn get_entry_parent_id() -> Option<EntryID> {
    // if parent
    // Some(EntryID(10))
    // else
    None
}

#[allow(dead_code)]
pub fn get_post(_post_id: EntryID) -> OX_Post {
    todo!()
}

impl OX_Post {
    pub fn draft(app: &mut App, entry_type: EntryType) -> Self {
        let the_post = Self {
            id: get_next_available_entry_id(app),
            id_author: crate::users::functions::get_current_user_id(&app),
            id_parent: get_entry_parent_id(),
            date_publish: date_functions::get_current_date(),
            date_modified: date_functions::get_current_date(),
            slug: None,
            the_type: entry_type,
            status: PostStatus::Draft,
            title: Some(consts::POST_UNTITLED_DEFAULT.to_string()),
            excerpt: None,
            content: None,
            password: None,
        };

        #[allow(clippy::let_unit_value)]
        let _store_post = store_post(&the_post);

        the_post
    }

    #[allow(dead_code)]
    pub fn update(self, _entry_data: Vec<String>) -> Self {
        todo!();
    }

    pub fn add_title(&mut self, title: String, create_permalink: bool) {
        let mut post_specifics_to_update: Vec<PostSpecific> = Vec::new();
        self.title = Some(title.clone());
        post_specifics_to_update.push(PostSpecific::Title);

        if create_permalink {
            self.add_permalink(title);
            post_specifics_to_update.push(PostSpecific::Permalink);
        }

        #[allow(clippy::let_unit_value)]
        let _update_post = update_post(self, post_specifics_to_update);
    }

    pub fn add_permalink(&mut self, slug: String) {
        let mut permalink_generator = url::permalink_generator::PermalinkGenerator::new(true);
        let slug = permalink_generator.create_permalink_from(slug);
        self.slug = Some(slug);
    }

    #[allow(dead_code)]
    pub fn add_content(&mut self, content: String) {
        self.content = Some(content);
    }

    #[allow(dead_code)]
    pub fn store(&mut self) -> bool {
        true
    }

    pub fn update_slug(&mut self, new_slug: &str) -> bool {
        self.slug = Some(new_slug.to_string());
        true
    }

    // Get current Entry ID
    #[allow(dead_code)]
    pub fn get_the_id(&self) -> EntryID {
        self.id.clone()
    }

    #[allow(dead_code)]
    pub fn get_author_id(&self) -> UserID {
        self.id_author.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_title_and_permalink_added() {
        let mut app = crate::app::App::init();

        let test_post_title: String = "Test title added".to_string();
        let test_post_slug: String = "test-title-added".to_string();

        let mut post = OX_Post::draft(&mut app, crate::posts::entry_type::EntryType::Post);
        post.add_title(test_post_title.clone(), true);

        assert_eq!(Some(test_post_title), post.title);
        assert_eq!(Some(test_post_slug), post.slug);
    }
}
