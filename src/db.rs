// Mock CSV database
use csv::{Reader, StringRecord, WriterBuilder};
use std::error::Error;

pub fn parse_csv(path: &str) -> Result<(), Box<dyn Error>> {
    println!("Parsing CSV: {path}");
    let mut reader = Reader::from_path(path)?;
    for row in reader.records() {
        dbg!(row);
        //let record = row?;
        // if let Some(user) = query_row.get(5)
    }
    Ok(())
}

pub fn write_to_csv(path: &str, post: &OX_Post) -> Result<(), Box<dyn Error>> {
    println!("Writing CSV: {path}");
    let mut writer = WriterBuilder::new().from_path(path)?;
    writer.write_record([
        post.id.to_string(),
        post.id_author.to_string(),
        post.id_parent.unwrap_or_default().to_string(),
        post.date_publish.to_string(),
        post.date_modified.to_string(),
        post.slug.clone().unwrap_or_default().to_string(),
        post.the_type.to_string(),
        post.title.clone().unwrap_or_default().to_string(),
        post.excerpt.clone().unwrap_or_default().to_string(),
        post.content.clone().unwrap_or_default().to_string(),
        post.password.clone().unwrap_or_default().to_string(),
    ])?;
    writer.flush()?;
    Ok(())
}

pub fn store(post: &OX_Post) {
    println!("Storing posts: {:?}", post);
    let _write = write_to_csv(consts::FILE_PATH, post);
}

use crate::consts;
use crate::posts::post::{EntryID, OX_Post};

pub fn add(post: &OX_Post) {}
pub fn update(post: &OX_Post) {}
pub fn delete(post: &OX_Post) {}
