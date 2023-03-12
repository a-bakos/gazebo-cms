use crate::allocator::ResourceID::{EntryID, UserID};
use crate::app::App;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ResourceType {
    User,
    Entry,
}

#[derive(Debug, PartialEq)]
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
    fn allocate(app: &mut App) -> Self;
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            allocated_ID: HashMap::new(),
        }
    }

    pub fn add_to_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {
        let mut resource_id = resource_id;
        if self.allocated_ID.get(&resource_type).is_some() {
            let id_list = self.allocated_ID.get_mut(&resource_type).unwrap();

            // Change ID if it's already in the list
            if id_list.contains(&resource_id) {
                resource_id = EntryID(300);
            }

            id_list.push(resource_id);
        } else {
            self.allocated_ID.insert(resource_type, vec![resource_id]);
        }
    }

    fn remove_from_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {
        todo!()
    }

    pub fn get_next_available_id(&self, resource_type: ResourceType) -> ResourceID {
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
