use crate::consts;
use crate::database::{columns, db};
use crate::entry::status::{EntryStatus, PostStatus};
use crate::entry::{
    entry_type::EntryType,
    post::{EntryID, GB_Post},
};
use crate::users::user::UserID;
use std::error::Error;

// Get post title
#[allow(dead_code)]
pub fn get_the_title(post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.title
}

// Get post URL
#[allow(dead_code)]
pub fn get_the_permalink(post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.slug
}

// Get post creation date
#[allow(dead_code)]
pub fn get_the_date(post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.date_publish)
}

// Get post modification date
#[allow(dead_code)]
pub fn get_the_modified_date(post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.date_modified)
}

// Get post content
#[allow(dead_code)]
pub fn get_the_content(post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.content
}

// Get post author
#[allow(dead_code)]
pub fn get_the_author(post_id: u32) -> Option<UserID> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.id_author)
}

// Get post parent
#[allow(dead_code)]
pub fn get_post_parent(post_id: u32) -> Option<EntryID> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.id_parent
}

// Get post type
#[allow(dead_code)]
pub fn get_post_type(post_id: u32) -> Option<EntryType> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.the_type)
}

// Get post excerpt
#[allow(dead_code)]
pub fn get_post_excerpt(post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.excerpt
}
