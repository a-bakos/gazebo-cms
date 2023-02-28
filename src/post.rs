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

#[derive(Debug)]
pub struct Entry {
    id: u32,
    id_author: u32,
    id_parent: u32,
    date_publish: String,
    date_modified: String,
    slug: String,
    post_type: EntryType,
    title: String,
    excerpt: String,
    content: String,
    password: String,
}

#[derive(Debug)]
pub enum EntryType {
    Post,
    Page,
    Media,
}

fn get_next_available_id() -> u32 {
    1
}

fn get_current_date() -> String {
    "2023-02-28 21:13".to_string()
}

impl Entry {
    pub fn create_empty(entry_type: EntryType) -> Self {
        Self {
            id: get_next_available_id(),
            id_author: 0,
            id_parent: 0,
            date_publish: get_current_date(),
            date_modified: get_current_date(),
            slug: "".to_string(),
            post_type: entry_type,
            title: "".to_string(),
            excerpt: "".to_string(),
            content: "".to_string(),
            password: "".to_string(),
        }
    }
}