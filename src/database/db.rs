use crate::consts;
use crate::database::consts::{
    DB_TABLE_ACCOUNTS, DB_TABLE_ACCOUNT_META, DB_TABLE_POSTS, DB_TABLE_POST_META,
};
use crate::posts::post::GB_Post;
use crate::users::user::User;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[allow(non_camel_case_types)]
pub enum DB_Table {
    Posts,
    PostMeta,
    Accounts,
    AccountMeta,
}
impl Into<String> for DB_Table {
    fn into(self) -> String {
        match self {
            DB_Table::Posts => DB_TABLE_POSTS.to_string(),
            DB_Table::PostMeta => DB_TABLE_POST_META.to_string(),
            DB_Table::Accounts => DB_TABLE_ACCOUNTS.to_string(),
            DB_Table::AccountMeta => DB_TABLE_ACCOUNT_META.to_string(),
        }
    }
}
impl Display for DB_Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DB_Table::Posts => write!(f, "{}", DB_TABLE_POSTS),
            DB_Table::PostMeta => write!(f, "{}", DB_TABLE_POST_META),
            DB_Table::Accounts => write!(f, "{}", DB_TABLE_ACCOUNTS),
            DB_Table::AccountMeta => write!(f, "{}", DB_TABLE_ACCOUNT_META),
        }
    }
}

pub struct GBDB {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub charset: String,
    pub table_prefix: String,
}

impl GBDB {
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
    pub fn get_row(db_table: DB_Table, _id: u32) {
        let path = match db_table {
            DB_Table::Accounts => Some(consts::FILE_PATH_USERS),
            DB_Table::PostMeta => None,
            DB_Table::Posts => Some(consts::FILE_PATH_POSTS),
            DB_Table::AccountMeta => None,
        };
        let csv = parse_csv(path.unwrap());
        for row in csv.unwrap().iter() {
            let result = crate::users::functions::turn_row_into_user(row);
            // todo!()
            dbg!(result);
            // row = StringRecord(["0", "First", "Last", "testuser", "test@test.com", "admin", "12345678", "2023-04-12 19:10:54"])
        }
    }
}

pub fn parse_csv(path: &str) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    println!("Parsing CSV: {path:?}");
    let mut csv_result: Vec<StringRecord> = Vec::new();
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    for row in reader.records() {
        let record = row?;
        csv_result.push(record);
    }
    Ok(csv_result)
}

pub fn write_post_to_csv(path: &str, the_post: &GB_Post) -> Result<(), Box<dyn Error>> {
    println!("Append post to CSV: {path:?}");
    dbg!(&the_post);
    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    let mut writer = WriterBuilder::new().from_writer(file);
    writer.write_record([
        the_post.id.to_string(),
        the_post.id_author.to_string(),
        the_post.id_parent.unwrap_or_default().to_string(),
        the_post.date_publish.to_string(),
        the_post.date_modified.to_string(),
        the_post.slug.clone().unwrap_or_default().to_string(),
        the_post.the_type.to_string(),
        the_post.title.clone().unwrap_or_default().to_string(),
        the_post.excerpt.clone().unwrap_or_default().to_string(),
        the_post.content.clone().unwrap_or_default().to_string(),
        the_post.password.clone().unwrap_or_default().to_string(),
    ])?;
    writer.flush()?;
    Ok(())
}

pub fn write_posts_to_csv(path: &str, the_posts: Vec<GB_Post>) -> Result<(), Box<dyn Error>> {
    println!("Writing CSV: {path:?}");
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
    let mut writer = WriterBuilder::new().from_writer(file);
    for the_post in the_posts.iter() {
        dbg!(&the_post);
        writer.write_record([
            the_post.id.to_string(),
            the_post.id_author.to_string(),
            the_post.id_parent.unwrap_or_default().to_string(),
            the_post.date_publish.to_string(),
            the_post.date_modified.to_string(),
            the_post.slug.clone().unwrap_or_default().to_string(),
            the_post.the_type.to_string(),
            the_post.title.clone().unwrap_or_default().to_string(),
            the_post.excerpt.clone().unwrap_or_default().to_string(),
            the_post.content.clone().unwrap_or_default().to_string(),
            the_post.password.clone().unwrap_or_default().to_string(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

pub fn write_users_to_csv(path: &str, user: &User) -> Result<(), Box<dyn Error>> {
    println!("Writing CSV: {path:?}");
    let mut writer = WriterBuilder::new().from_path(path)?;

    writer.write_record([
        user.id.to_string(),
        user.login_name.to_string(),
        user.email.to_string(),
        user.role.to_string(),
        user.password.to_string(),
        user.registered.to_string(),
    ])?;

    writer.flush()?;
    Ok(())
}

pub fn store_post(the_post: &GB_Post) {
    let _write = write_post_to_csv(consts::FILE_PATH_POSTS, the_post);
}

pub fn store_all_posts(the_posts: Vec<GB_Post>) {
    println!("Storing posts: {the_posts:?}");
    let _write = write_posts_to_csv(consts::FILE_PATH_POSTS, the_posts);
}

pub fn store_user(user: &User) {
    println!("Storing user: {user:?}");
    let _write = write_users_to_csv(consts::FILE_PATH_USERS, user);
}

use crate::posts::post::PostSpecific;

pub fn update_post(post: &GB_Post, post_specs_to_update: Vec<PostSpecific>) -> bool {
    // get post by id
    let search_index;
    let mut all_posts = crate::posts::functions::get_all_posts().unwrap();
    dbg!(&all_posts);
    for single_post in all_posts.iter() {
        if single_post.id == post.id {
            search_index = post.id;
            // create post struct from row
            let mut replacement_post = GB_Post {
                id: single_post.id.clone(),
                id_author: single_post.id_author.clone(),
                id_parent: single_post.id_parent.clone(),
                date_publish: single_post.date_publish.clone(),
                date_modified: crate::datetime::functions::get_current_date(),
                slug: single_post.slug.clone(),
                the_type: single_post.the_type.clone(),
                status: single_post.status.clone(),
                title: single_post.title.clone(),
                excerpt: single_post.excerpt.clone(),
                content: single_post.content.clone(),
                password: single_post.password.clone(),
            };

            for spec in post_specs_to_update.iter() {
                match spec {
                    PostSpecific::Title => replacement_post.title = post.title.clone(),
                    PostSpecific::Permalink => replacement_post.slug = post.slug.clone(),
                    PostSpecific::AuthorID => replacement_post.id_author = post.id_author.clone(),
                    PostSpecific::ParentID => replacement_post.id_parent = post.id_parent.clone(),
                    PostSpecific::Excerpt => replacement_post.excerpt = post.excerpt.clone(),
                    PostSpecific::Content => replacement_post.content = post.content.clone(),
                    PostSpecific::Password => replacement_post.password = post.password.clone(),
                }
            }

            if let Some(index) = all_posts.iter().position(|item| -> bool {
                let the_post_id = item.id.clone();
                the_post_id == search_index
            }) {
                all_posts.remove(index);
            }

            //single post drop
            dbg!(&all_posts);
            //replacement_post push
            all_posts.push(replacement_post);
            dbg!(&all_posts);
            store_all_posts(all_posts);

            return true;
        }
        break;
    }
    return false;
}

#[allow(dead_code)]
pub fn add(_post: &GB_Post) {}
#[allow(dead_code)]
pub fn delete(_post: &GB_Post) {}
