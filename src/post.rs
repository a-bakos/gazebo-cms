/*
WP_POST
    [x] public $ID;
    [x] public $post_author = 0;
    [x] public $post_date = '0000-00-00 00:00:00'; // The post's local publication time.
    public $post_date_gmt = '0000-00-00 00:00:00'; // The post's GMT publication time.
    [x] public $post_content = '';
    [x] public $post_title = '';
    [x] public $post_excerpt = '';
    [x] public $post_status = 'publish';
    public $comment_status = 'open';
    public $ping_status = 'open'; // Whether pings are allowed
    [x] public $post_password = ''; // The post's password in plain text.
    [x] public $post_name = ''; // The post's slug.
    public $to_ping = ''; // URLs queued to be pinged.
    public $pinged = ''; // URLs that have been pinged.
    [x] public $post_modified = '0000-00-00 00:00:00'; // The post's local modified time.
    public $post_modified_gmt = '0000-00-00 00:00:00'; // The post's GMT modified time.
    public $post_content_filtered = ''; // A utility DB field for post content.
    [x] public $post_parent = 0; // ID of a post's parent post.
    public $guid = ''; // The unique identifier for a post, not necessarily a URL, used as the feed GUID.
    public $menu_order = 0; // A field used for ordering posts.
    [x] public $post_type = 'post';
    public $post_mime_type = ''; // An attachment's mime type.
    public $comment_count = 0; // Cached comment count. A numeric string, for compatibility reasons.
*/

use crate::allocator::{ID_Allocator, ResourceID, ResourceManager, ResourceType};
use crate::user::{User, UserID};
use crate::{consts, date};

#[derive(Debug)]
pub struct OX_Post {
    id: EntryID,
    id_author: UserID,
    id_parent: Option<EntryID>,
    date_publish: String,
    date_modified: String,
    slug: Option<String>,
    the_type: EntryType,
    title: Option<String>,
    excerpt: Option<String>,
    content: Option<String>,
    password: Option<String>,
}

#[derive(Debug)]
pub enum EntryType {
    Post,
    Page,
    Media,
}

// New type patterns for IDs
#[derive(Debug)]
pub struct EntryID(u32);

impl EntryID {
    // get current ID
    fn get() -> Self {
        EntryID(200)
    }
}

impl ID_Allocator for EntryID {
    fn allocate() -> Self {
        // resourcemanager to allocate entry ID
        EntryID(200)
    }
}

// Get current Entry ID
fn get_the_id() -> EntryID {
    EntryID::get()
}

fn get_author_id() -> UserID {
    UserID(100)
}

fn get_next_available_entry_id() -> EntryID {
    EntryID::allocate()
}

fn get_entry_parent_id() -> Option<EntryID> {
    // if parent
    // Some(EntryID(10))
    // else
    None
}

pub fn get_post(post_id: EntryID) -> OX_Post {
    todo!()
}

impl OX_Post {
    pub fn draft(entry_type: EntryType) -> Self {
        Self {
            id: get_next_available_entry_id(),
            id_author: get_author_id(),
            id_parent: get_entry_parent_id(),
            date_publish: date::get_current_date(),
            date_modified: date::get_current_date(),
            slug: None,
            the_type: entry_type,
            title: None,
            excerpt: None,
            content: None,
            password: None,
        }
    }

    pub fn update(mut self, entryData: Vec<String>) -> Self {
        todo!();
    }

    pub fn add_title(&mut self, title: String, create_permalink: bool) {
        self.title = Some(title.clone());

        if create_permalink {
            let permalink = title.to_lowercase();
            self.add_permalink(permalink);
        }
    }

    pub fn add_permalink(&mut self, slug: String) {
        let slug = slug.replace(" ", consts::PERMALINK_SEPARATOR);
        self.slug = Some(slug);
    }

    pub fn store(&mut self) -> bool {
        true
    }
}
