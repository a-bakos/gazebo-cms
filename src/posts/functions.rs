use crate::consts;
use crate::database::{columns, db};
use crate::posts::post::PostStatus;
use crate::posts::{
    entry_type::EntryType,
    post::{EntryID, OX_Post},
};
use crate::users::user::UserID;
use std::error::Error;

fn turn_row_into_post(row: &csv::StringRecord) -> OX_Post {
    // Turn into OX_Post
    OX_Post {
        id: EntryID(
            row.get(columns::COL_INDEX_POST_ID)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        id_author: UserID(
            row.get(columns::COL_INDEX_POST_ID_AUTHOR)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        id_parent: None,
        date_publish: row
            .get(columns::COL_INDEX_POST_DATE_PUBLISH)
            .unwrap()
            .to_string(),
        date_modified: row
            .get(columns::COL_INDEX_POST_DATE_MODIFIED)
            .unwrap()
            .to_string(),
        slug: Some(row.get(columns::COL_INDEX_POST_SLUG).unwrap().to_string()),
        the_type: EntryType::Post,
        status: PostStatus::Draft,
        title: Some(row.get(columns::COL_INDEX_POST_TITLE).unwrap().to_string()),
        excerpt: None,
        content: None,
        password: None,
    }
}

pub fn get_all_posts() -> Result<Vec<OX_Post>, Box<dyn Error>> {
    let mut posts: Vec<OX_Post> = Vec::new();
    let csv_db = db::parse_csv(consts::FILE_PATH_POSTS)?;
    for post in csv_db.iter() {
        let the_post = turn_row_into_post(post);
        posts.push(the_post);
    }

    Ok(posts)
}

pub fn get_post_by_id(post_id: u32) -> Result<Option<OX_Post>, Box<dyn Error>> {
    let csv_db = db::parse_csv(consts::FILE_PATH_POSTS)?;
    let found_post;
    let mut post = None;
    for row in csv_db.iter() {
        if let Some(id) = row.get(columns::COL_INDEX_POST_ID) {
            if id == post_id.to_string() {
                found_post = row;
                post = Some(turn_row_into_post(found_post));
                break;
            }
        }
    }

    Ok(post)
}

// Get post title
#[allow(dead_code)]
pub fn get_the_title(post_id: u32) -> Option<String> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    post.title
}

// Get post URL
#[allow(dead_code)]
pub fn get_the_permalink(post_id: u32) -> Option<String> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    post.slug
}

// Get post creation date
#[allow(dead_code)]
pub fn get_the_date(post_id: u32) -> Option<String> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    Some(post.date_publish)
}

// Get post modification date
#[allow(dead_code)]
pub fn get_the_modified_date(post_id: u32) -> Option<String> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    Some(post.date_modified)
}

// Get post content
#[allow(dead_code)]
pub fn get_the_content(post_id: u32) -> Option<String> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    post.content
}

// Get post author
#[allow(dead_code)]
pub fn get_the_author(post_id: u32) -> Option<UserID> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    Some(post.id_author)
}

// Get post parent
#[allow(dead_code)]
pub fn get_post_parent(post_id: u32) -> Option<EntryID> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    post.id_parent
}

// Get post type
#[allow(dead_code)]
pub fn get_post_type(post_id: u32) -> Option<EntryType> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    Some(post.the_type)
}

// Get post excerpt
#[allow(dead_code)]
pub fn get_post_excerpt(post_id: u32) -> Option<String> {
    let post = get_post_by_id(post_id).unwrap().unwrap();
    post.excerpt
}
