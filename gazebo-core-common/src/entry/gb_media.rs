pub struct GB_Media {
    id: u32,
    uploader: u32,
    attached_to: Vec<u32>,
    date_publish: String,
    date_modified: String,
    slug: String,
    title: String,
    description: Option<String>,
    alt_text: Option<String>,
}
