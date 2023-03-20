// Mock CSV database
use crate::consts;
use crate::posts::post::OX_Post;
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
