use crate::allocator::ResourceManager;

pub(crate) struct App {
    name: String,
    resources: ResourceManager,
}

impl App {
    fn new(app_name: String) -> Self {
        Self {
            name: app_name,
            resources: ResourceManager::new(),
        }
    }

    pub(crate) fn init(app_name: String) -> Self {
        App::new(app_name)
    }
}