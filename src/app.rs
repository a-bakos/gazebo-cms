use crate::allocator::ResourceManager;
use crate::database::db::Database;
use crate::users::user::UserID;
use crate::users::user_manager::UserManager;
use std::time::SystemTime;

// Todo need to implement Debug for App
// #[derive(Debug)]
pub struct App {
    pub name: String,
    pub version: String,
    pub resources: ResourceManager,
    #[allow(dead_code)]
    db: Database,
    pub start: SystemTime,
    #[allow(dead_code)]
    debug_mode: bool,
    // logged in users
    pub users: Vec<String>,
    current_user: Option<UserID>, // this is just an idea. it doesn't have an effect until we go async...
}

impl App {
    fn new(app_name: String, version: String) -> Self {
        Self {
            name: app_name,
            version,
            resources: ResourceManager::new(), // HashMap<ResourceType, Vec<ResourceID>>
            db: Database::new(
                "database".to_string(),
                "user".to_string(),
                "pass".to_string(),
                "host".to_string(),
                "charset".to_string(),
                "prefix_".to_string(),
            ),
            start: SystemTime::now(),
            debug_mode: false,
            users: Vec::new(),
            current_user: None,
        }
    }

    pub fn init(app_name: String, version: String) -> Self {
        App::new(app_name, version)
    }
}
