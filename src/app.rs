use crate::allocator::ResourceManager;
use crate::db::Database;

pub struct App {
    pub name: String,
    pub resources: ResourceManager,
    db: Database,
}

impl App {
    fn new(app_name: String) -> Self {
        Self {
            name: app_name,
            resources: ResourceManager::new(), // HashMap<ResourceType, Vec<ResourceID>>
            db: Database::new(
                "database".to_string(),
                "user".to_string(),
                "pass".to_string(),
                "host".to_string(),
                "charset".to_string(),
                "prefix_".to_string(),
            ),
        }
    }

    pub fn init(app_name: String) -> Self {
        App::new(app_name)
    }
}
