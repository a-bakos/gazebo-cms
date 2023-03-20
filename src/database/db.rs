use std::collections::HashMap;
// Mock CSV database
use crate::consts;
use crate::posts::entry_type::EntryType;
use crate::posts::post::{EntryID, OX_Post};
use crate::users::user::UserID;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::error::Error;

pub struct Database {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub charset: String,
    pub table_prefix: String,
}

impl Database {
    pub fn new(
        name: String,
        user: String,
        password: String,
        host: String,
        charset: String,
        table_prefix: String,
    ) -> Self {
        Self {
            name,
            user,
            password,
            host,
            charset,
            table_prefix,
        }
    }

    #[allow(dead_code)]
    pub fn get_row(_id: u32) {
        todo!()
    }
}

pub fn parse_csv(path: &str) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    println!("Parsing CSV: {:?}", path);
    let mut csv_result: Vec<StringRecord> = Vec::new();
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    for row in reader.records() {
        let record = row?;
        csv_result.push(record);
    }
    Ok(csv_result)
}

pub fn write_to_csv(path: &str, posts: Vec<&OX_Post>) -> Result<(), Box<dyn Error>> {
    println!("Writing CSV: {:?}", path);
    let mut writer = WriterBuilder::new().from_path(path)?;
    for single_post in posts.iter() {
        writer.write_record([
            single_post.id.to_string(),
            single_post.id_author.to_string(),
            single_post.id_parent.unwrap_or_default().to_string(),
            single_post.date_publish.to_string(),
            single_post.date_modified.to_string(),
            single_post.slug.clone().unwrap_or_default().to_string(),
            single_post.the_type.to_string(),
            single_post.title.clone().unwrap_or_default().to_string(),
            single_post.excerpt.clone().unwrap_or_default().to_string(),
            single_post.content.clone().unwrap_or_default().to_string(),
            single_post.password.clone().unwrap_or_default().to_string(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

pub fn store(posts: Vec<&OX_Post>) {
    println!("Storing posts: {:?}", posts);
    let _write = write_to_csv(consts::FILE_PATH, posts);
}

#[allow(dead_code)]
pub fn add(_post: &OX_Post) {}
#[allow(dead_code)]
pub fn update(_post: &OX_Post) {}
#[allow(dead_code)]
pub fn delete(_post: &OX_Post) {}

pub const COL_ID: usize = 0;
pub const COL_ID_AUTHOR: usize = 1;
pub const COL_PARENT: usize = 2;
pub const COL_DATE_PUBLISH: usize = 3;
pub const COL_DATE_MODIFIED: usize = 4;
pub const COL_SLUG: usize = 5;
pub const COL_POST_TYPE: usize = 6;
pub const COL_TITLE: usize = 7;
pub const COL_EXCERPT: usize = 8;
pub const COL_CONTENT: usize = 9;
pub const COL_PASSWORD: usize = 10;

pub fn get_columns() -> HashMap<String, usize> {
    let mut columns: HashMap<String, usize> = HashMap::new();
    columns.insert(COL_ID.to_string(), COL_ID);
    columns.insert(COL_ID_AUTHOR.to_string(), COL_ID_AUTHOR);
    columns.insert(COL_PARENT.to_string(), COL_PARENT);
    columns.insert(COL_DATE_PUBLISH.to_string(), COL_DATE_PUBLISH);
    columns.insert(COL_DATE_MODIFIED.to_string(), COL_DATE_MODIFIED);
    columns.insert(COL_SLUG.to_string(), COL_SLUG);
    columns.insert(COL_POST_TYPE.to_string(), COL_POST_TYPE);
    columns.insert(COL_TITLE.to_string(), COL_TITLE);
    columns.insert(COL_EXCERPT.to_string(), COL_EXCERPT);
    columns.insert(COL_CONTENT.to_string(), COL_CONTENT);
    columns.insert(COL_PASSWORD.to_string(), COL_PASSWORD);
    columns
}
