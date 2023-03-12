use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ResourceType {
    User,
    Entry,
}

#[derive(Debug)]
pub enum ResourceID {
    UserID(u32),
    EntryID(u32),
}

#[derive(Debug)]
pub struct ResourceManager {
    allocated_ID: HashMap<ResourceType, Vec<ResourceID>>,
}

#[allow(non_camel_case_types)]
pub trait ID_Allocator {
    fn allocate() -> Self;
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            allocated_ID: HashMap::new(),
        }
    }

    fn add_to_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {
        if self.allocated_ID.get(&resource_type).is_some() {
            let id_list = self.allocated_ID.get_mut(&resource_type).unwrap();
            id_list.push(resource_id);
        } else {
            self.allocated_ID.insert(resource_type, vec![resource_id]);
        }
    }

    fn remove_from_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {
        todo!()
    }

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
