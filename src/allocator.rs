use crate::app::App;
use crate::consts;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ResourceType {
    User,
    Entry,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ResourceID {
    UserID(u32),
    EntryID(u32),
}

impl From<ResourceID> for u32 {
    fn from(resource_id: ResourceID) -> Self {
        match resource_id {
            ResourceID::UserID(id) => id,
            ResourceID::EntryID(id) => id,
        }
    }
}

impl<'a> From<&'a ResourceID> for u32 {
    fn from(resource_id: &'a ResourceID) -> Self {
        match resource_id {
            ResourceID::UserID(id) => *id,
            ResourceID::EntryID(id) => *id,
        }
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ResourceManager {
    allocated_ID: HashMap<ResourceType, Vec<ResourceID>>,
    last_allocated_ID: HashMap<ResourceType, ResourceID>,
}

#[allow(non_camel_case_types)]
pub trait ID_Allocator {
    fn allocate(app: &mut App) -> Self;
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            allocated_ID: HashMap::new(),
            last_allocated_ID: HashMap::new(),
        }
    }

    pub fn add_to_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {
        // If resource type exists
        if self.allocated_ID.get(&resource_type).is_some() {
            let id_list: &mut Vec<ResourceID> = self.allocated_ID.get_mut(&resource_type).unwrap();

            if id_list.contains(&resource_id) {
                unreachable!()
            }

            id_list.push(resource_id);
            self.last_allocated_ID.insert(resource_type, resource_id);
        } else {
            self.allocated_ID.insert(resource_type, vec![resource_id]);
            self.last_allocated_ID.insert(resource_type, resource_id);
        }
    }

    #[allow(dead_code)]
    fn remove_from_allocated(&mut self, _resource_type: ResourceType, _resource_id: ResourceID) {
        todo!()
    }

    pub fn get_next_available_id(app: &App, resource_type: ResourceType) -> ResourceID {
        match resource_type {
            ResourceType::User => {
                // acquire next available ID
                ResourceID::UserID(2)
            }
            ResourceType::Entry => {
                // Check if we have last allocated ID stored, increment if yes, otherwise start from 1
                if let Some(id) = app.resources.last_allocated_ID.get(&resource_type) {
                    let try_id: u32 = u32::from(id) + 1;

                    if app.resources.allocated_ID.get(&resource_type).is_some() {
                        ResourceID::EntryID(try_id)
                    } else {
                        ResourceID::EntryID(0)
                    }
                } else {
                    ResourceID::EntryID(consts::ID_START_VALUE)
                }
            }
        }
    }
}
