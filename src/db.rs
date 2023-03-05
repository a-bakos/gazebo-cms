// Mock database
use crate::consts;
use crate::post::OX_Post;
use csv::WriterBuilder;
use std::error::Error;

pub fn store(post: &OX_Post) -> Result<(), Box<dyn Error>> {
    println!("Writing post data into CSV...");
    let mut wtr = WriterBuilder::new().from_path(consts::FILE_PATH)?;

    //

    wtr.write_record(["1".to_string(), "2".to_string()])?;
    wtr.flush()?;
    Ok(())
}
