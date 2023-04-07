use crate::app::App;
use crate::consts;
use crate::database::{columns, db};
use crate::posts::entry_type::EntryType;
use crate::posts::post::{EntryID, OX_Post};
use crate::users::user::UserID;
use std::error::Error;

fn turn_row_into_post(row: &csv::StringRecord) -> OX_Post {
    // Turn into OX_Post
    OX_Post {
        id: EntryID(
            row.get(columns::COL_INDEX_ID)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        id_author: UserID(
            row.get(columns::COL_INDEX_ID_AUTHOR)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        id_parent: None,
        date_publish: row
            .get(columns::COL_INDEX_DATE_PUBLISH)
            .unwrap()
            .to_string(),
        date_modified: row
            .get(columns::COL_INDEX_DATE_MODIFIED)
            .unwrap()
            .to_string(),
        slug: Some(row.get(columns::COL_INDEX_SLUG).unwrap().to_string()),
        the_type: EntryType::Post,
        title: Some(row.get(columns::COL_INDEX_TITLE).unwrap().to_string()),
        excerpt: None,
        content: None,
        password: None,
    }
}

pub fn get_all_posts() -> Result<Vec<OX_Post>, Box<dyn Error>> {
    let mut posts: Vec<OX_Post> = Vec::new();
    let csv_db = db::parse_csv(consts::FILE_PATH)?;
    for post in csv_db.iter() {
        let the_post = turn_row_into_post(post);
        posts.push(the_post);
    }

    Ok(posts)
}

pub fn get_post_by_id(_app: &App, post_id: u32) -> Result<Option<OX_Post>, Box<dyn Error>> {
    let csv_db = db::parse_csv(consts::FILE_PATH)?;
    let found_post;
    let mut post = None;
    for row in csv_db.iter() {
        if let Some(id) = row.get(columns::COL_INDEX_ID) {
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
pub fn get_the_title(_app: &App, _post_id: u32) -> Option<String> {
    Some(String::new())
}

// Get post URL
pub fn get_the_permalink(_app: &App, _post_id: u32) -> Option<String> {
    Some(String::new())
}

// Get post creation date
pub fn get_the_date(_app: &App, _post_id: u32) -> Option<String> {
    Some(String::new())
}

// Get post modification date
pub fn get_the_modified_date(_app: &App, _post_id: u32) -> Option<String> {
    Some(String::new())
}

// Get post content
pub fn get_the_content(_app: &App, _post_id: u32) -> Option<String> {
    Some(String::new())
}

// Get post author
pub fn get_the_author(_app: &App, _post_id: u32) -> Option<String> {
    Some(String::new())
}
