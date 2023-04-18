use crate::allocator::ResourceManager;
use crate::database::db::Database;
use crate::users::user::UserID;
use std::time::SystemTime;

// Todo need to implement Debug for App
// #[derive(Debug)]
pub struct App {
    pub name: String,
    pub admin_email: String,
    pub version: String,
    pub resources: ResourceManager,
    #[allow(dead_code)]
    pub(crate) db: Database,
    pub start: SystemTime,
    #[allow(dead_code)]
    debug_mode: bool,
    // logged in users
    pub users: Vec<String>,
    pub current_user: Option<UserID>, // this is just an idea. it doesn't have an effect until we go async...
}

impl App {
    fn new() -> Self {
        Self {
            name: crate::consts::DEFAULT_APP_NAME.to_string(),
            admin_email: crate::consts::DEFAULT_APP_ADMIN_EMAIL.to_string(),
            version: crate::consts::VERSION.to_string(),
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

    pub fn init() -> Self {
        App::new()
    }

    pub fn change_admin_email(&mut self, new_admin_email: &str) -> bool {
        self.admin_email = new_admin_email.to_owned();
        true
    }

    pub fn change_app_name(&mut self, new_app_name: &str) -> bool {
        self.name = new_app_name.to_owned();
        true
    }
}
