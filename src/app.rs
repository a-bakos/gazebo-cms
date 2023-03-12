use crate::allocator::ResourceManager;

pub struct App {
    pub name: String,
    pub resources: ResourceManager,
}

impl App {
    fn new(app_name: String) -> Self {
        Self {
            name: app_name,
            resources: ResourceManager::new(), // HashMap<ResourceType, Vec<ResourceID>>
        }
    }

    pub fn init(app_name: String) -> Self {
        App::new(app_name)
    }
}