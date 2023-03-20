use crate::consts;
use crate::database::{columns, db};
use crate::posts::entry_type::EntryType;
use crate::posts::post::{EntryID, OX_Post};
use crate::users::user::UserID;
use std::error::Error;

pub fn get_post_by_id(post_id: u32) -> Result<Option<OX_Post>, Box<dyn Error>> {
    let csv_db = db::parse_csv(consts::FILE_PATH)?;
    let found_post;
    let mut post = None;
    for row in csv_db.iter() {
        if let Some(id) = row.get(columns::COL_INDEX_ID) {
            if id == post_id.to_string() {
                found_post = row;

                // Turn into OX_Post
                post = Some(OX_Post {
                    id: EntryID(
                        found_post
                            .get(columns::COL_INDEX_ID)
                            .unwrap()
                            .parse::<u32>()
                            .unwrap(),
                    ),
                    id_author: UserID(
                        found_post
                            .get(columns::COL_INDEX_ID_AUTHOR)
                            .unwrap()
                            .parse::<u32>()
                            .unwrap(),
                    ),
                    id_parent: None,
                    date_publish: found_post
                        .get(columns::COL_INDEX_DATE_PUBLISH)
                        .unwrap()
                        .to_string(),
                    date_modified: found_post
                        .get(columns::COL_INDEX_DATE_MODIFIED)
                        .unwrap()
                        .to_string(),
                    slug: Some(found_post.get(columns::COL_INDEX_SLUG).unwrap().to_string()),
                    the_type: EntryType::Post,
                    title: Some(
                        found_post
                            .get(columns::COL_INDEX_TITLE)
                            .unwrap()
                            .to_string(),
                    ),
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
