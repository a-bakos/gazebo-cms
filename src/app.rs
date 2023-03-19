use crate::allocator::ResourceManager;
use crate::database::db::Database;

pub struct App {
    pub name: String,
    pub version: String,
    pub resources: ResourceManager,
    #[allow(dead_code)]
    db: Database,
    // todo: start of run timestamp
    // todo: debug mode
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
        }
    }

    pub fn init(app_name: String, version: String) -> Self {
        App::new(app_name, version)
    }
}
