use crate::{
    app::App,
    consts,
    datetime::functions as date_functions,
    entry::{
        entry_type::EntryType,
        status::{EntryStatus, PostStatus},
    },
    url,
    users::user::UserID,
};

use crate::users::user::User;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

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
pub struct GB_PostItem {
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

// New type patterns for IDs
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct EntryID(pub u32);

impl std::fmt::Display for EntryID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for EntryID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[allow(dead_code)]
fn get_entry_parent_id() -> Option<EntryID> {
    // if parent
    // Some(EntryID(10))
    // else
    None
}

#[allow(dead_code)]
pub fn get_post(_post_id: EntryID) -> GB_PostItem {
    todo!()
}

impl GB_PostItem {
    pub fn draft(app: &mut App) -> Self {
        Self {
            id: EntryID(1),
            id_author: UserID(1),
            id_parent: get_entry_parent_id(),
            date_publish: date_functions::get_current_date(),
            date_modified: date_functions::get_current_date(),
            slug: None,
            status: EntryStatus::Post(PostStatus::Draft),
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

    pub fn add_permalink(&mut self, slug: String) {
        let mut permalink_generator = url::permalink_generator::PermalinkGenerator::new(true);
        let slug = permalink_generator.create_permalink_from(slug);
        self.slug = Some(slug);
    }

    #[allow(dead_code)]
    pub fn add_content(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn update_slug(&mut self, new_slug: &str) -> bool {
        self.slug = Some(new_slug.to_string());
        true
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

        let mut post = GB_PostItem::draft(&mut app);
        post.add_title(test_post_title.clone(), true);

        //assert_eq!(Some(test_post_title), post.title);
        //assert_eq!(Some(test_post_slug), post.slug);
    }
}
