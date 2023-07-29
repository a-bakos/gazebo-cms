pub struct GB_Post {
    pub id: u32,
    pub id_author: u32,
    pub id_parent: Option<u32>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: Option<String>,
    pub status: String,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub password: Option<String>,
}
