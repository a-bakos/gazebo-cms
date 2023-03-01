use std::collections::HashMap;

pub enum ResourceType {
    User,
    Entry,
}

pub enum ResourceID {
    UserID(u32),
    EntryID(u32),
}

pub struct ResourceManager {
    allocated_ID: HashMap<ResourceType, Vec<ResourceID>>,
}

pub trait ID_Allocator {
    fn allocate() -> Self;
}

impl ResourceManager {
    fn add_to_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {}
    fn remove_from_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {}

    pub fn get_next_available_id(resource_type: ResourceType) -> ResourceID {
        match resource_type {
            ResourceType::User => {
                // acquire next available ID
                ResourceID::UserID(2)
            }
            ResourceType::Entry => {
                // acquire next available ID
                ResourceID::EntryID(1)
            }
        }
    }
}
