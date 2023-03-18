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
use crate::dates::date_functional;
use crate::posts::entry_type::EntryType;
use crate::url;
use crate::users::user::UserID;
use std::fmt::Formatter;

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
            _ => EntryID(0),
        };
        let _ = &app
            .resources
            .add_to_allocated(ResourceType::Entry, resource_entry_id);
        entry_id
    }
}

// Get current Entry ID
#[allow(dead_code)]
fn get_the_id() -> EntryID {
    EntryID::get()
}

#[allow(dead_code)]
fn get_author_id() -> UserID {
    UserID(100)
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
        Self {
            id: get_next_available_entry_id(app),
            id_author: get_author_id(),
            id_parent: get_entry_parent_id(),
            date_publish: date_functional::get_current_date(),
            date_modified: date_functional::get_current_date(),
            slug: None,
            the_type: entry_type,
            title: None,
            excerpt: None,
            content: None,
            password: None,
        }
    }

    #[allow(dead_code)]
    pub fn update(self, _entry_data: Vec<String>) -> Self {
        todo!();
    }

    pub fn add_title(&mut self, title: String, create_permalink: bool) {
        self.title = Some(title.clone());

        if create_permalink {
            self.add_permalink(title);
        }
    }

    pub fn add_permalink(&mut self, slug: String) {
        let slug = url::permalink::create_permalink_from(slug);
        self.slug = Some(slug);
    }

    #[allow(dead_code)]
    pub fn store(&mut self) -> bool {
        true
    }
}
