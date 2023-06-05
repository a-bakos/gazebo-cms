// Mock DB (table) files
pub const FILE_PATH_POSTS: &str = "mock_db_posts.csv";
pub const FILE_PATH_USERS: &str = "mock_db_users.csv";

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
    println!("Storing entry: {the_posts:?}");
    let _write = write_posts_to_csv(consts::FILE_PATH_POSTS, the_posts);
}

pub fn store_user(user: &User) {
    println!("Storing user: {user:?}");
    let _write = write_users_to_csv(consts::FILE_PATH_USERS, user);
}

pub fn update_post(post: &GB_Post, post_specs_to_update: Vec<PostSpecific>) -> bool {
    // get post by id
    let search_index;
    let mut all_posts = crate::entry::functions::get_all_posts().unwrap();
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

// GBDB Impl
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

#[allow(dead_code)]
pub fn get_next_available_user_id(app: &mut App) -> UserID {
    UserID::allocate(app)
}

pub fn turn_row_into_user(row: &csv::StringRecord) -> User {
    User {
        login_name: row.get("login".parse().unwrap()).unwrap().to_string(),
        email: row.get("email".parse().unwrap()).unwrap().to_string(),
        id: UserID(
            row.get("id".parse().unwrap())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        role: crate::users::roles::get_role_variant(row.get("role".parse().unwrap()).unwrap()),
        password: row.get("password".parse().unwrap()).unwrap().to_string(),
        registered: row.get("registered".parse().unwrap()).unwrap().to_string(),
    }
}

fn turn_row_into_post(row: &csv::StringRecord) -> GB_Post {
    // Turn into OX_Post
    GB_Post {
        id: EntryID(
            row.get(columns::COL_INDEX_POST_ID)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        id_author: UserID(
            row.get(columns::COL_INDEX_POST_ID_AUTHOR)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        id_parent: None,
        date_publish: row
            .get(columns::COL_INDEX_POST_DATE_PUBLISH)
            .unwrap()
            .to_string(),
        date_modified: row
            .get(columns::COL_INDEX_POST_DATE_MODIFIED)
            .unwrap()
            .to_string(),
        slug: Some(row.get(columns::COL_INDEX_POST_SLUG).unwrap().to_string()),
        the_type: EntryType::Post,
        status: EntryStatus::PostStatus(PostStatus::Draft),
        title: Some(row.get(columns::COL_INDEX_POST_TITLE).unwrap().to_string()),
        excerpt: None,
        content: None,
        password: None,
    }
}

pub fn get_all_posts() -> Result<Vec<GB_Post>, Box<dyn Error>> {
    let mut posts: Vec<GB_Post> = Vec::new();
    let csv_db = db::parse_csv(consts::FILE_PATH_POSTS)?;
    for post in csv_db.iter() {
        let the_post = turn_row_into_post(post);
        posts.push(the_post);
    }

    Ok(posts)
}

pub fn get_post_by_id(post_id: u32) -> Result<Option<GB_Post>, Box<dyn Error>> {
    let csv_db = db::parse_csv(consts::FILE_PATH_POSTS)?;
    let found_post;
    let mut post = None;
    for row in csv_db.iter() {
        if let Some(id) = row.get(columns::COL_INDEX_POST_ID) {
            if id == post_id.to_string() {
                found_post = row;
                post = Some(turn_row_into_post(found_post));
                break;
            }
        }
    }

    Ok(post)
}

pub fn get_user_by_email(email: &str) -> Result<Option<User>, Box<dyn Error>> {
    // todo: if is valid email
    //if !is_email_valid(&email) {
    //    return an error
    //}

    let csv_db = db::parse_csv(consts::FILE_PATH_USERS)?;
    let found_user;
    let mut user = None;
    for row in csv_db.iter() {
        if let Some(db_email) = row.get("email".parse().unwrap()) {
            if db_email.to_lowercase() == email.to_lowercase() {
                found_user = row;
                user = Some(turn_row_into_user(found_user));
                break;
            }
        }
    }

    Ok(user)
}

pub fn user_exists(email: &str) -> bool {
    if get_user_by_email(email).is_ok() && get_user_by_email(email).unwrap().is_some() {
        return true;
    }
    false
}
