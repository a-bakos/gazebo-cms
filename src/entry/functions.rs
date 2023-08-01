use crate::{
    entry::{entry_id::EntryID, entry_type::EntryType},
    users::user::UserID,
};

// Get post URL
#[allow(dead_code)]
pub fn get_the_permalink(_post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.slug
}

// Get post creation date
#[allow(dead_code)]
pub fn get_the_date(_post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.date_publish)
}

// Get post modification date
#[allow(dead_code)]
pub fn get_the_modified_date(_post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.date_modified)
}

// Get post content
#[allow(dead_code)]
pub fn get_the_content(_post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.content
}

// Get post author
#[allow(dead_code)]
pub fn get_the_author(_post_id: u32) -> Option<UserID> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.id_author)
}

// Get post parent
#[allow(dead_code)]
pub fn get_post_parent(_post_id: u32) -> Option<EntryID> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.id_parent
}

// Get post type
#[allow(dead_code)]
pub fn get_post_type(_post_id: u32) -> Option<EntryType> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // Some(post.the_type)
}

// Get post excerpt
#[allow(dead_code)]
pub fn get_post_excerpt(_post_id: u32) -> Option<String> {
    todo!()
    // let post = get_post_by_id(post_id).unwrap().unwrap();
    // post.excerpt
}
