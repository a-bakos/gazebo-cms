use crate::consts;
use crate::database::db;
use crate::posts::entry_type::EntryType;
use crate::posts::post::{EntryID, OX_Post};
use crate::users::user::UserID;
use std::error::Error;

// move this to post functions.rs
pub fn get_post_by_id(post_id: u32) -> Result<Option<OX_Post>, Box<dyn Error>> {
    let csv_db = db::parse_csv(consts::FILE_PATH)?;
    let found_post;
    let mut post = None;
    for row in csv_db.iter() {
        if let Some(id) = row.get(0) {
            if id == post_id.to_string() {
                found_post = row;

                // Turn into OX_Post
                post = Some(OX_Post {
                    id: EntryID(found_post.get(0).unwrap().parse::<u32>().unwrap()),
                    id_author: UserID(found_post.get(1).unwrap().parse::<u32>().unwrap()),
                    id_parent: None,
                    date_publish: found_post.get(3).unwrap().to_string(),
                    date_modified: found_post.get(4).unwrap().to_string(),
                    slug: Some(found_post.get(5).unwrap().to_string()),
                    the_type: EntryType::Post,
                    title: Some(found_post.get(7).unwrap().to_string()),
                    excerpt: None,
                    content: None,
                    password: None,
                });
                break;
            }
        }
    }

    Ok(post)
}
