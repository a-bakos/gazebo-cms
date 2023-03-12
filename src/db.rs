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

pub fn write_to_csv(path: &str) -> Result<(), Box<dyn Error>> {
    println!("Writing CSV: {path}");
    let mut writer = WriterBuilder::new().from_path(path)?;
    writer.write_record(["TEST K ".to_string(), "TEST V ".to_string()])?;
    writer.flush()?;
    Ok(())
}

use crate::consts;
use crate::post::{EntryID, OX_Post};

pub fn add(post: &OX_Post) {}
pub fn update(post: &OX_Post) {}
pub fn delete(post: &OX_Post) {}
