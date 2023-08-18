use crate::{
    app::App,
    consts,
    database::columns::{
        COL_INDEX_POST_CONTENT, COL_INDEX_POST_DATE_MODIFIED, COL_INDEX_POST_DATE_PUBLISH,
        COL_INDEX_POST_EXCERPT, COL_INDEX_POST_ID, COL_INDEX_POST_ID_AUTHOR, COL_INDEX_POST_PARENT,
        COL_INDEX_POST_SLUG, COL_INDEX_POST_STATUS, COL_INDEX_POST_TITLE,
    },
    traits::RowTransformer,
    url,
    url::permalink_generator::PermalinkGenerator,
};

use gazebo_core_common::{
    account::gb_account::AccountID,
    consts::POST_UNTITLED_DEFAULT_TITLE,
    entry::{
        entry_id::EntryID,
        entry_type::EntryType,
        gb_post::GB_Post,
        status::{get_entry_status_variant, ContentStatus, EntryStatus},
    },
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

impl RowTransformer<PgRow> for GB_Post {
    type Output = GB_Post;

    fn transform(row: &PgRow) -> Self::Output {
        // Underscores' meaning here:
        // we don't need to specify a default/fallback value because the cell will never be empty

        // IDs
        let post_id = row.get::<i32, _>(COL_INDEX_POST_ID) as u32;
        let author_id = row.get::<i32, _>(COL_INDEX_POST_ID_AUTHOR) as u32;
        let parent_id = row
            .try_get(COL_INDEX_POST_PARENT)
            .ok()
            .unwrap_or(consts::ENTRY_ID_NO_PARENT) as u32;

        // Publish date
        let date_publish: NaiveDateTime = row.get::<NaiveDateTime, _>(COL_INDEX_POST_DATE_PUBLISH);
        let date_publish = date_publish.to_string();

        // Modified date
        let date_modified: NaiveDateTime =
            row.get::<NaiveDateTime, _>(COL_INDEX_POST_DATE_MODIFIED);
        let date_modified = date_modified.to_string();

        // Entry status
        let entry_status_as_str: &str = row.get(COL_INDEX_POST_STATUS);
        let status: EntryStatus = get_entry_status_variant(entry_status_as_str, &EntryType::Post);

        Self {
            id: EntryID(post_id),
            id_author: AccountID(author_id),
            id_parent: Some(EntryID(parent_id)),
            date_publish,
            date_modified,
            slug: row.get(COL_INDEX_POST_SLUG),
            status,
            title: row.get(COL_INDEX_POST_TITLE),
            excerpt: row.get(COL_INDEX_POST_EXCERPT),
            content: row.get(COL_INDEX_POST_CONTENT),
            password: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_title_and_permalink_added() {
        let mut app = App::init();

        let test_post_title: String = "Test title added".to_string();
        let test_post_slug: String = "test-title-added".to_string();

        let mut post = GB_Post::draft(&mut app);
        post.add_title(test_post_title.clone(), true);

        //assert_eq!(Some(test_post_title), post.title);
        //assert_eq!(Some(test_post_slug), post.slug);
    }
}
