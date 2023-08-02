use crate::{
    app::App,
    consts,
    database::columns::{
        COL_INDEX_POST_CONTENT, COL_INDEX_POST_DATE_MODIFIED, COL_INDEX_POST_DATE_PUBLISH,
        COL_INDEX_POST_EXCERPT, COL_INDEX_POST_ID, COL_INDEX_POST_ID_AUTHOR, COL_INDEX_POST_PARENT,
        COL_INDEX_POST_SLUG, COL_INDEX_POST_STATUS, COL_INDEX_POST_TITLE,
    },
    datetime::functions as date_functions,
    entry::{
        entry_id,
        entry_id::EntryID,
        entry_type::EntryType,
        status::{get_entry_status_variant, ContentStatus, EntryStatus},
    },
    traits::RowTransformer,
    url,
    users::user::UserID,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug)]
#[allow(dead_code)]
pub enum PostSpecific {
    Title,
    Permalink,
    AuthorID,
    ParentID,
    // DatePublished,
    Excerpt,
    Content,
    Password,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GB_Post {
    pub id: EntryID,
    pub id_author: UserID,
    pub id_parent: Option<EntryID>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: Option<String>,
    pub status: EntryStatus,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub password: Option<String>,
}

impl GB_Post {
    #[allow(dead_code)]
    pub fn draft(_app: &mut App) -> Self {
        Self {
            id: EntryID(1),
            id_author: UserID(1),
            id_parent: entry_id::get_entry_parent_id(),
            date_publish: date_functions::get_current_date(),
            date_modified: date_functions::get_current_date(),
            slug: None,
            status: EntryStatus::Post(ContentStatus::Draft),
            title: Some(consts::POST_UNTITLED_DEFAULT_TITLE.to_string()),
            excerpt: None,
            content: None,
            password: None,
        }
    }

    #[allow(dead_code)]
    pub fn update(self, _entry_data: Vec<String>) -> Self {
        todo!();
    }

    #[allow(dead_code)]
    pub fn add_title(&mut self, title: String, create_permalink: bool) {
        let mut post_specifics_to_update: Vec<PostSpecific> = Vec::new();
        self.title = Some(title.clone());
        post_specifics_to_update.push(PostSpecific::Title);

        if create_permalink {
            self.add_permalink(title);
            post_specifics_to_update.push(PostSpecific::Permalink);
        }

        // #[allow(clippy::let_unit_value)]
        // let _update_post = update_post(self, post_specifics_to_update);
    }

    #[allow(dead_code)]
    pub fn add_permalink(&mut self, slug: String) {
        let mut permalink_generator = url::permalink_generator::PermalinkGenerator::new(true);
        let slug = permalink_generator.create_permalink_from(slug);
        self.slug = Some(slug);
    }

    #[allow(dead_code)]
    pub fn add_content(&mut self, content: String) {
        self.content = Some(content);
    }

    #[allow(dead_code)]
    pub fn update_slug(&mut self, new_slug: &str) -> bool {
        self.slug = Some(new_slug.to_string());
        true
    }
}

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
            id_author: UserID(author_id),
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
